use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    const MOD: u32 = 1_000_000_007;

    let op = "atcoder";
    let mut dp = vec![0; op.len() + 1];
    for c in s.chars() {
        if let Some(x) = op.find(c) {
            dp[x + 1] = (dp[x] + dp[x + 1]) % MOD;
        }
    }
}
