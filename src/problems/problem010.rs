//! [10] Summation of primes
//! ------------------------
//!
//! ### Problem
//! ```
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//! Find the sum of all the primes below two million.
//! ```
use utils;
use utils::Sum;

pub fn solve() -> u64 {
    let limit = 2_000_000;
    utils::primes().take_while(|&n| n < limit).summation()
}
