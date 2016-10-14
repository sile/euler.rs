//! [50] Consecutive prime sum
//! --------------------------
//!
//! https://projecteuler.net/problem=50
//!
use euler_lib::utils;

pub fn solve() -> u64 {
    let limit = 1_000_000;
    let primes: Vec<_> = utils::primes().take_while(|&n| n < limit).collect();
    let mut acc = vec![0; primes.len()];
    let mut max = (0, 0); // (consecutive_coutn, number)
    let mut start_j = 0;

    for i in 0..primes.len() {
        for j in start_j..i + 1 {
            acc[j] += primes[i];
            let consecutives = i - j;
            if max.0 < consecutives {
                if primes.binary_search_by(|x| x.cmp(&acc[j])).is_ok() {
                    max = (consecutives, acc[j]);
                }
            }
            while acc[start_j] >= limit {
                start_j += 1;
            }
        }
    }

    max.1
}
