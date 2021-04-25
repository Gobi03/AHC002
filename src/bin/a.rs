#[allow(unused_imports)]
use proconio::marker::Chars;
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

use rand::{thread_rng, Rng};
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

#[fastout]
fn main() {
    let system_time = SystemTime::now();

    input! {
        // n: usize,
        // v: [isize; n],
    }

    let ans = 42;

    println!("{}", ans);

    println!("{}ms", system_time.elapsed().unwrap().as_millis());
}
