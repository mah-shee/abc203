#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if a == c {
        print!("{}", b);
    } else {
        println!("0");
    }
}
