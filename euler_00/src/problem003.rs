//! [3] Largest prime factor
//! ------------------------
//!
use euler_lib::utils;

pub fn solve() -> u64 {
    let num = 600851475143;
    utils::primes().take_while(|n| n * n < num).filter(|n| num % n == 0).last().unwrap()
}
