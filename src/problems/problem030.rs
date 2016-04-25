//! [30] Digit fifth powers
//! -----------------------
//!
//! https://projecteuler.net/problem=30
//!
use num;
use utils::Sum;

const POWERS: usize = 5;

pub fn solve() -> usize {
    (2..)
        .take_while(|&n| n < num::pow(9, POWERS) * format!("{}", n).len())
        .filter(|&n| powers_of_digit(n) == n)
        .summation()
}

fn powers_of_digit(n: usize) -> usize {
    let s = format!("{}", n);
    s.chars().map(|c| num::pow(c.to_digit(10).unwrap() as usize, POWERS)).summation()
}
