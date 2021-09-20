#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }
    // (1..k).sum() * n + (1..n).sum() * 100 * k
    let ans: usize = (1..=k).sum::<usize>() * n + (1..=n).sum::<usize>() * 100 * k;
    println!("{}", ans);
}
