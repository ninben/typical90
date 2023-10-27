use proconio::marker::*;
use proconio::*;
fn main() {
    input! {
        n: usize,
        e:[(Usize1,Usize1); n-1],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in e {
        g[a].push(b);
        g[b].push(a);
    }

    let mut root = 0;
    let mut ans = 0;

    for _ in 0..3 {
        let mut dp = vec![n; n];
        dp[root] = 0;
        let mut dfs = vec![(root, root)];
        while let Some((v, p)) = dfs.pop() {
            for &u in g[v].iter() {
                if u != p {
                    dfs.push((u, v));
                    dp[u] = dp[v] + 1;
                }
            }
        }
        let (a, &b) = dp.iter().enumerate().max_by_key(|p| p.1).unwrap();
        ans = b;
        root = a;
    }
    println!("{}", ans + 1);
}
