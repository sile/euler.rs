//! [7] 10001st prime
//! -----------------
//!
//! ### Problem
//! ```text
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//! What is the 10 001st prime number?
//! ```
use euler_lib::utils;

pub fn solve() -> u64 {
    let nth = 10_001;
    utils::primes().nth(nth - 1).unwrap()
}
