//! [37] Truncatable primes
//! -----------------------
//!
//! https://projecteuler.net/problem=37
//!
use num;
use utils::{self, Sum};
use std::collections::HashSet;

pub fn solve() -> u64 {
    let mut primes = HashSet::new();
    utils::primes()
        .filter(|&n| {
            primes.insert(n);
            is_truncatable_prime(n, &primes)
        })
        .skip_while(|&n| n < 10)
        .take(11)
        .summation()
}

pub fn is_truncatable_prime(n: u64, primes: &HashSet<u64>) -> bool {
    let (mut x, mut y) = num::integer::div_rem(n, 10);
    let mut digits = 1;
    while x > 0 {
        if !primes.contains(&x) || !primes.contains(&y) {
            return false;
        }
        let t = num::integer::div_rem(x, 10);
        x = t.0;
        y += t.1 * num::pow(10, digits);
        digits += 1;
    }
    true
}
