//! [3] Largest prime factor
//! ------------------------
//!
//! ### Problem
//! ```
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?
//! ```
use utils;

pub fn solve() -> u64 {
    let num = 600851475143;
    utils::primes().take_while(|n| n * n < num).filter(|n| num % n == 0).last().unwrap()
}
