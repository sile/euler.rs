//! [27] Quadratic primes
//! ----------------------
//!
//! https://projecteuler.net/problem=27
//!
use utils::{self,Prime};
use std::collections::HashSet;

struct Primes {
    prime_seq: Prime,
    prime_set: HashSet<u64>,
    last: u64,
}

impl Primes {
    pub fn is_prime(&mut self, n: isize) -> bool {
        if n <= 1 { return false }
        if self.last < n as u64 {
            while let Some(p) = self.prime_seq.next() {
                self.prime_set.insert(p);
                self.last = p;
                if p > n as u64 { break; }
            }
        }
        self.prime_set.contains(&(n as u64))
    }
}

fn primes() -> Primes { Primes{prime_seq: utils::primes(), prime_set: HashSet::new(), last: 0 } }

pub fn solve() -> isize {
    let mut primes = primes();
    (-999..1000).map(|a| {
        (-999..1000).map(|b| {
            (quadratic_prime_len(a, b, &mut primes),  a * b)
        }).max().unwrap()
    }).max().unwrap().1
}


fn quadratic_prime_len(a: isize, b: isize, primes: &mut Primes) -> usize {
    (0..).take_while(|n| primes.is_prime(n*n + a*n + b) ).count()
}
