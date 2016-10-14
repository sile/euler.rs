//! [30] Digit fifth powers
//! -----------------------
//!
//! https://projecteuler.net/problem=30
//!
use num;

const POWERS: usize = 5;

pub fn solve() -> u64 {
    (2..)
        .take_while(|&n| n < num::pow(9, POWERS) * format!("{}", n).len() as u64)
        .filter(|&n| powers_of_digit(n) == n)
        .sum()
}

fn powers_of_digit(n: u64) -> u64 {
    let s = format!("{}", n);
    s.chars().map(|c| num::pow(c.to_digit(10).unwrap() as u64, POWERS)).sum()
}
