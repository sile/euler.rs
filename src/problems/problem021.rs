//! [21] Amicable numbers
//! -----------------------
//!
//! ### Problem
//! ```
//! Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
//! If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
//!
//! For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//!
//! Evaluate the sum of all the amicable numbers under 10000.
//! ```
use utils::{self, Sum};

pub fn solve() -> usize {
    let limit = 10_000;
    (2..limit).filter(|&n| is_amicable(n) ).summation()
}

fn is_amicable(n: usize) -> bool {
    let x = utils::divisors(n).summation();
    x != 1 && x != n && n == utils::divisors(x).summation()
}
