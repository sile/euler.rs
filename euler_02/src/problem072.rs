//! [72] Counting fractions
//! -----------------------
//!
//! see also: problem070.rs
use euler_lib::utils;
use num::rational::Ratio;

const LIMIT: usize = 1_000_001;

pub fn solve() -> u64 {
    let mut table = vec![Ratio::new(1,1); LIMIT];
    for p in utils::primes().map(|p| p as usize).take_while(|&p| p < LIMIT) {
        let r = Ratio::new(p, p - 1);
        for i in (1..LIMIT / p + 1).map(|i| i * p) {
            if i >= table.len() {
                continue;
            }
            table[i] = table[i] * r.clone();
        }
    }
    table.into_iter()
        .zip(0..)
        .skip(2)
        .map(|(r, n)| totient(n, &r) as u64)
        .sum()
}

fn totient(n: usize, r: &Ratio<usize>) -> usize {
    let t = Ratio::new(*r.denom() * n, *r.numer());
    assert!(t.is_integer());
    t.to_integer()
}
