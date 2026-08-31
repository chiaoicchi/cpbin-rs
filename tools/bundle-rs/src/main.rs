use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (lib, input) = match args.as_slice() {
        [f, lib, input] if f == "--lib" => (PathBuf::from(lib), PathBuf::from(input)),
        _ => {
            eprintln!("usage: bundle-rs --lib <crate-root> <file.rs>");
            std::process::exit(2);
        }
    };
    match run(&lib, &input) {
        Ok(out) => print!("{out}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run(lib: &Path, input: &Path) -> Result<String, String> {
    let name = pkg_name(lib)?;
    let src = lib.join("src");
    let sol = read(input)?;

    let mut queue = VecDeque::new();
    for p in scan(&sol, &name) {
        match resolve(&src, &p) {
            Some(mp) => queue.push_back(mp),
            None => eprintln!("warning: cannot resolve `{}::{}`", name, p.join("::")),
        }
    }
    if queue.is_empty() {
        return Ok(sol);
    }

    let mut files: HashMap<Vec<String>, String> = HashMap::new();
    while let Some(mp) = queue.pop_front() {
        if files.contains_key(&mp) {
            continue;
        }
        let text = strip_comments(&read(&file_of(&src, &mp).unwrap())?);
        for p in scan(&text, "crate") {
            match resolve(&src, &p) {
                Some(dep) => queue.push_back(dep),
                None => eprintln!(
                    "warning: cannot resolve `crate::{}` in {}",
                    p.join("::"),
                    mp.join("::")
                ),
            }
        }
        files.insert(mp, text);
    }

    let mut tree = BTreeSet::new();
    for mp in files.keys() {
        for k in 0..=mp.len() {
            tree.insert(mp[..k].to_vec());
        }
    }

    let mut out = sol;
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("\n#[allow(dead_code, unused_imports)]\n");
    out.push_str(&format!("mod {name} {{\n"));
    emit(&mut out, &[], &files, &tree, &name, 1);
    out.push_str("}\n");
    Ok(out)
}

fn idc(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn ws(s: &[u8], i: &mut usize) {
    while *i < s.len() && s[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn ident(s: &[u8], i: &mut usize) -> Option<String> {
    if *i >= s.len() || !idc(s[*i]) || s[*i].is_ascii_digit() {
        return None;
    }
    let st = *i;
    while *i < s.len() && idc(s[*i]) {
        *i += 1;
    }
    Some(String::from_utf8(s[st..*i].to_vec()).unwrap())
}

fn scan(text: &str, prefix: &str) -> Vec<Vec<String>> {
    let needle = format!("{prefix}::");
    let s = text.as_bytes();
    let mut out = vec![];
    for (p, _) in text.match_indices(&needle) {
        if p > 0 && idc(s[p - 1]) {
            continue;
        }
        let mut i = p + needle.len();
        out.extend(parse(s, &mut i));
    }
    out
}

fn parse(s: &[u8], i: &mut usize) -> Vec<Vec<String>> {
    ws(s, i);
    if *i < s.len() && s[*i] == b'{' {
        *i += 1;
        let mut out = vec![];
        loop {
            ws(s, i);
            if *i >= s.len() {
                break;
            }
            if s[*i] == b'}' {
                *i += 1;
                break;
            }
            out.extend(parse(s, i));
            let mut d = 0;
            while *i < s.len() {
                match s[*i] {
                    b'{' => d += 1,
                    b'}' if d == 0 => break,
                    b'}' => d -= 1,
                    b',' if d == 0 => break,
                    _ => {}
                }
                *i += 1;
            }
            if *i < s.len() && s[*i] == b',' {
                *i += 1;
            }
        }
        out
    } else if *i < s.len() && s[*i] == b'*' {
        *i += 1;
        vec![vec![]]
    } else if let Some(id) = ident(s, i) {
        if id == "self" {
            vec![vec![]]
        } else if s[*i..].starts_with(b"::") {
            *i += 2;
            let tails = parse(s, i);
            if tails.is_empty() {
                vec![vec![id]]
            } else {
                tails
                    .into_iter()
                    .map(|mut t| {
                        t.insert(0, id.clone());
                        t
                    })
                    .collect()
            }
        } else {
            vec![vec![id]]
        }
    } else {
        vec![]
    }
}

fn rewrite(line: &str, name: &str) -> String {
    let s = line.as_bytes();
    let mut out = String::new();
    let mut done = 0;
    for (p, _) in line.match_indices("crate::") {
        if p > 0 && idc(s[p - 1]) {
            continue;
        }
        out.push_str(&line[done..p + 7]);
        out.push_str(name);
        out.push_str("::");
        done = p + 7;
    }
    out.push_str(&line[done..]);
    out
}

fn mod_decl(line: &str) -> Option<String> {
    let s = line.trim().as_bytes();
    let mut i = 0;
    let mut head = ident(s, &mut i)?;
    if head == "pub" {
        ws(s, &mut i);
        if i < s.len() && s[i] == b'(' {
            while i < s.len() && s[i] != b')' {
                i += 1;
            }
            if i >= s.len() {
                return None;
            }
            i += 1;
            ws(s, &mut i);
        }
        head = ident(s, &mut i)?;
    }
    if head != "mod" {
        return None;
    }
    ws(s, &mut i);
    let name = ident(s, &mut i)?;
    ws(s, &mut i);
    (i < s.len() && s[i] == b';').then_some(name)
}

fn resolve(src: &Path, segs: &[String]) -> Option<Vec<String>> {
    for k in (1..=segs.len()).rev() {
        if file_of(src, &segs[..k]).is_some() {
            return Some(segs[..k].to_vec());
        }
    }
    file_of(src, &[]).map(|_| vec![])
}

fn file_of(src: &Path, mp: &[String]) -> Option<PathBuf> {
    if mp.is_empty() {
        let p = src.join("lib.rs");
        return p.is_file().then_some(p);
    }
    let mut dir = src.to_path_buf();
    for s in &mp[..mp.len() - 1] {
        dir.push(s);
    }
    let last = mp.last().unwrap();
    [
        dir.join(format!("{last}.rs")),
        dir.join(last).join("mod.rs"),
    ]
    .into_iter()
    .find(|p| p.is_file())
}

fn pkg_name(lib: &Path) -> Result<String, String> {
    let text = read(&lib.join("Cargo.toml"))?;
    let mut in_pkg = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_pkg = t == "[package]";
        } else if in_pkg {
            if let Some(v) = t
                .strip_prefix("name")
                .map(str::trim_start)
                .and_then(|r| r.strip_prefix('='))
            {
                return Ok(v.trim().trim_matches('"').replace('-', "_"));
            }
        }
    }
    Err("no [package] name in Cargo.toml".to_string())
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn strip_comments(text: &str) -> String {
    text.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .map(|l| format!("{l}\n"))
        .collect()
}

fn emit(
    out: &mut String,
    node: &[String],
    files: &HashMap<Vec<String>, String>,
    tree: &BTreeSet<Vec<String>>,
    name: &str,
    depth: usize,
) {
    let ind = "    ".repeat(depth);
    let mut seen = BTreeSet::new();
    if let Some(text) = files.get(node) {
        let mut bd = 0i64;
        let mut attrs: Vec<&str> = vec![];
        for line in text.lines() {
            let t = line.trim();
            if bd == 0 && t.starts_with("#[") && t.ends_with(']') {
                attrs.push(t);
                continue;
            }
            if bd == 0 {
                if let Some(m) = mod_decl(t) {
                    seen.insert(m.clone());
                    let child = [node, &[m.clone()][..]].concat();
                    if tree.contains(&child) {
                        flush(out, &mut attrs, &ind, name);
                        out.push_str(&format!("{ind}pub mod {m} {{\n"));
                        emit(out, &child, files, tree, name, depth + 1);
                        out.push_str(&format!("{ind}}}\n"));
                    } else {
                        attrs.clear();
                    }
                    continue;
                }
            }
            flush(out, &mut attrs, &ind, name);
            if t.is_empty() {
                out.push('\n');
            } else {
                out.push_str(&ind);
                out.push_str(&rewrite(line, name));
                out.push('\n');
            }
            bd += line
                .chars()
                .fold(0, |d, c| d + (c == '{') as i64 - (c == '}') as i64);
        }
        flush(out, &mut attrs, &ind, name);
    }
    for child in tree
        .iter()
        .filter(|p| p.len() == node.len() + 1 && p.starts_with(node))
    {
        let m = child.last().unwrap();
        if !seen.contains(m) {
            out.push_str(&format!("{ind}pub mod {m} {{\n"));
            emit(out, child, files, tree, name, depth + 1);
            out.push_str(&format!("{ind}}}\n"));
        }
    }
}

fn flush(out: &mut String, attrs: &mut Vec<&str>, ind: &str, name: &str) {
    for a in attrs.drain(..) {
        out.push_str(ind);
        out.push_str(&rewrite(a, name));
        out.push('\n');
    }
}
