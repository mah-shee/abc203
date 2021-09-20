#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: u128,
        mut a_b: [(u128, u128); n]
    }
    a_b.sort_by_key(|j| j.0);
    let mut now = 0;
    for i in 0..n {
        if a_b[i].0 <= now + k {
            k = k - (a_b[i].0 - now) + a_b[i].1;
            now = a_b[i].0;
        } else {
            break;
        }
    }
    println!("{}", now + k);
}
