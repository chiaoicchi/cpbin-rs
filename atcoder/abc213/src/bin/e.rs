use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
    };
    let mut heap = std::collections::BinaryHeap::new();
    let mut dist = vec![vec![!0usize; w]; h];
    heap.push((!0, (0usize, 0usize)));
    dist[0][0] = 0;
    while let Some((d, (i, j))) = heap.pop() {
        let d = !d;
        if dist[i][j] < d {
            continue;
        }
        for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if ni < h && nj < w && s[ni][nj] == b'.' && d < dist[ni][nj] {
                dist[ni][nj] = d;
                heap.push((!d, (ni, nj)));
            }
        }
        for di in [!1, !0, 0, 1, 2] {
            for dj in [!1, !0, 0, 1, 2] {
                if [!1, 2].contains(&di) && [!1, 2].contains(&dj) {
                    continue;
                }
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < h && nj < w && d + 1 < dist[ni][nj] {
                    dist[ni][nj] = d + 1;
                    heap.push((!(d + 1), (ni, nj)));
                }
            }
        }
    }
    println!("{}", dist[h - 1][w - 1]);
}
