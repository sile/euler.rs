//! [77] Prime summations
//! ---------------------
//!
use euler_lib::utils::{self, Prime};
use std::collections::HashSet;

// TODO: problem027と共通化
struct Primes {
    prime_seq: Prime,
    prime_set: HashSet<u64>,
    last: u64,
}

impl Primes {
    pub fn is_prime(&mut self, n: usize) -> bool {
        if n <= 1 {
            return false;
        }
        if self.last < n as u64 {
            while let Some(p) = self.prime_seq.next() {
                self.prime_set.insert(p);
                self.last = p;
                if p > n as u64 {
                    break;
                }
            }
        }
        self.prime_set.contains(&(n as u64))
    }
}

fn primes() -> Primes {
    Primes {
        prime_seq: utils::primes(),
        prime_set: HashSet::new(),
        last: 0,
    }
}

pub fn solve() -> u64 {
    let limit = 5_000;
    let mut primes = primes();
    let mut ways_table: Vec<Vec<_>> = Vec::new();
    (0..)
        .find(|n| {
            let mut ways = vec![0; n+1];
            for lft in 2..n / 2 + 1 {
                let rgt = n - lft;
                if primes.is_prime(lft) && primes.is_prime(rgt) {
                    ways[lft] += 1;
                }
                if primes.is_prime(lft) {
                    ways[lft] += ways_table[rgt][lft..].iter().fold(0, |x, y| x + y);
                }
            }
            ways_table.push(ways);
            ways_table.last().unwrap().iter().fold(0, |x, y| x + y) > limit
        })
        .unwrap() as u64
}
