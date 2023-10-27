use proconio::*;
use std::cmp;

fn main() {
    input! {
      n: usize, l: i32,
      k: i32,
      a: [i32; n]
    }

    let mut hi = l;
    let mut lo = 0;

    while hi - lo > 1 {
        let mid = hi + lo >> 1;
        let mut left = 0;
        let mut cnt = 0;
        for i in 0..n {
            if a[i] - left >= mid {
                cnt += 1;
                left = a[i];
            }
        }
        if cnt > k || (cnt == k && l - left >= mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    println!("{}", lo);
}
