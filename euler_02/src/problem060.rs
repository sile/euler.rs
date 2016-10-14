//! [60] Prime pair sets
//! --------------------
//!
use num;
use euler_lib::utils::{self, Prime};
use std::collections::HashSet;

// TODO: 共通化
struct Primes {
    prime_seq: Prime,
    prime_set: HashSet<u64>,
    last: u64,
}

impl Primes {
    pub fn is_prime(&mut self, n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if self.last < n {
            while let Some(p) = self.prime_seq.next() {
                self.prime_set.insert(p);
                self.last = p;
                if p > n {
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

fn concat(x: u64, y: u64) -> u64 {
    x * num::pow(10, utils::digits(y)) + y
}

fn is_prime_pair(x: u64, y: u64, primes: &mut Primes) -> bool {
    primes.is_prime(concat(x, y)) && primes.is_prime(concat(y, x))
}

pub fn solve() -> u64 {
    let primes = &mut primes();
    let mut traversed = Vec::new();
    for a in utils::primes() {
        for &b in traversed.iter() {
            if is_prime_pair(a, b, primes) {
                for &c in traversed.iter().take_while(|&&c| c < b) {
                    if is_prime_pair(a, c, primes) && is_prime_pair(b, c, primes) {
                        for &d in traversed.iter().take_while(|&&d| d < c) {
                            if is_prime_pair(a, d, primes) && is_prime_pair(b, d, primes) &&
                               is_prime_pair(c, d, primes) {
                                for &e in traversed.iter().take_while(|&&e| e < d) {
                                    if is_prime_pair(a, e, primes) && is_prime_pair(b, e, primes) &&
                                       is_prime_pair(c, e, primes) &&
                                       is_prime_pair(d, e, primes) {
                                        return a + b + c + d + e;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        traversed.push(a);
    }
    unreachable!()
}
