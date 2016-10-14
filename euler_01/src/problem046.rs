//! [46] Goldbach's other conjecture
//! --------------------------------
//!
//! https://projecteuler.net/problem=46
//!
use euler_lib::utils;
use num::integer::Integer;

pub fn solve() -> u64 {
    utils::composites()
        .filter(|n| n.is_odd())
        .filter(|&n| !is_goldbach(n))
        .nth(0)
        .unwrap()
}

fn is_goldbach(n: u64) -> bool {
    utils::primes().take_while(|&p| p < n).any(|p| {
        let mut x = n - p;
        if x.is_odd() {
            return false;
        }

        x /= 2;
        (1..).take_while(|&y| y * y <= x).any(|y| y * y == x)
    })
}
