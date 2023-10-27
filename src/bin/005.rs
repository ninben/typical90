use proconio::*;

fn main() {
    input! {
        n:usize, p:usize, k:usize,
        c:[usize;k],
    }
    const MOD: u64 = 1_000_000_007;

    type DP = (Vec<u64>, usize);
    let mul = |a: &DP, b: &DP| -> DP {
        let mut c = vec![0; p];
        for (i, &x) in a.0.iter().enumerate() {
            for (j, &y) in b.0.iter().enumerate() {
                let po = &mut c[(i * b.1 + j) % p];
                *po = (*po + x * y) % MOD;
            }
        }
        (c, a.1 * b.1 % p);
    };
    let mut t = (vec![0; p], 1);
    t.0[0] = 1;
    let mut s = (vec![0; p], 10 % p);
    for c in c {
        s.0[c % p] += 1;
    }
    while n > 0 {
        if n & 1 == 1 {
            t = mul(&t, &s);
        }
        s = mul(&s, &s);
        n >>= 1;
    }
    let ans = t.0[0];
    println!("{}", ans);
}
