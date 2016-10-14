//! [35] Circular primes
//! --------------------
//!
//! https://projecteuler.net/problem=35
//!
use num;
use euler_lib::utils;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() -> u64 {
    let primes = HashSet::from_iter(utils::primes().take_while(|&n| n < 1_000_000));
    primes.iter().filter(|&&n| is_circular_prime(n, &primes)).count() as u64
}

fn is_circular_prime(mut n: u64, primes: &HashSet<u64>) -> bool {
    let c = digit_count(n);
    for _ in 0..c {
        if !primes.contains(&n) {
            return false;
        }
        let (div, rem) = num::integer::div_rem(n, 10);
        n = rem * num::pow(10, c - 1) + div;
    }
    true
}

fn digit_count(mut n: u64) -> usize {
    let mut c = 1;
    while n > 9 {
        n /= 10;
        c += 1;
    }
    c
}
