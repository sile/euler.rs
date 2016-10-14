//! [21] Amicable numbers
//! -----------------------
//!
//! ### Problem
//! ```text
//! Let d(n) be defined as the sum of proper divisors of
//! n (numbers less than n which divide evenly into n).
//!
//! If d(a) = b and d(b) = a, where a ≠ b, then a and b are
//! an amicable pair and each of a and b are called amicable numbers.
//!
//! For example, the proper divisors of 220 are
//! 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
//! therefore d(220) = 284. The proper divisors of 284 are
//! 1, 2, 4, 71 and 142; so d(284) = 220.
//!
//! Evaluate the sum of all the amicable numbers under 10000.
//! ```
use euler_lib::utils;

pub fn solve() -> u64 {
    let limit: u64 = 10_000;
    (2..limit).filter(|&n| is_amicable(n)).sum()
}

fn is_amicable(n: u64) -> bool {
    let x: u64 = utils::divisors(n as usize).map(|a| a as u64).sum();
    let y: u64 = utils::divisors(x as usize).map(|a| a as u64).sum();
    x != 1 && x != n && n == y
}
