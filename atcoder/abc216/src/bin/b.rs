use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, String); n],
    };
    st.sort();
    st.dedup();
    println!("{}", if n == st.len() { "No" } else { "Yes" });
}
