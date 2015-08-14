//! [51] Prime digit replacements
//! -----------------------------
//!
//! https://projecteuler.net/problem=51
//!
use num;
use utils::{self,Prime};
use std::collections::HashSet;

// TODO: move to utils module (also problem027)
struct Primes {
    prime_seq: Prime,
    prime_set: HashSet<u64>,
    last: u64,
}

impl Primes {
    pub fn is_prime(&mut self, n: u64) -> bool {
        if n <= 1 { return false }
        if self.last < n as u64 {
            while let Some(p) = self.prime_seq.next() {
                self.prime_set.insert(p);
                self.last = p;
                if p > n as u64 { break; }
            }
        }
        self.prime_set.contains(&n)
    }
}

fn primes() -> Primes { Primes{prime_seq: utils::primes(), prime_set: HashSet::new(), last: 0 } }

pub fn solve() -> u64 {
    let mut result = 0;
    let mut primes = primes();

    'root: for p in utils::primes().take(1_000_000) {
        let ds = utils::digits(p as usize);
        let mut indices = Vec::new();
        for i in (0..ds-1) {
            if get_digit_nth(p, i) > 2 { continue }
            gen_indices(p, get_digit_nth(p, i), i+1, ds, &mut vec![i], &mut indices);
        }
        for index in indices {
            let count = replaced_prime_count(p, get_digit_nth(p, index[0]), &index, &mut primes);
            if count == 8 {
                result = p;
                break 'root;
            }
        }
    }
    result
}

fn gen_indices(p: u64, d: u8, i: usize, len: usize, v: &mut Vec<usize>, indices: &mut Vec<Vec<usize>>) {
    if i == len { return }

    if get_digit_nth(p, i) == d {
        v.push(i);
        indices.push(v.clone());
        gen_indices(p, d, i+1, len, v, indices);
        v.pop();
    }
    gen_indices(p, d, i+1, len, v, indices);
}

fn replaced_prime_count(mut p: u64, s: u8, indices: &[usize], primes: &mut Primes) -> usize {
    (s+1..10).filter(|&d| {
        for &i in indices.iter() {
            set_digit_nth(&mut p, i, d);
        }
        primes.is_prime(p)
    }).count() + 1
}

fn get_digit_nth(n: u64, i: usize) -> u8 {
    let m = n / num::pow(10, i);
    assert!(m != 0);
    (m % 10) as u8
}

fn set_digit_nth(n: &mut u64, i: usize, d: u8) {
    let old = get_digit_nth(*n, i);
    let delta = (d as i8) - (old as i8);
    let m = (*n as i64) + (delta as i64) * num::pow(10, i);
    *n = m as u64;
}
