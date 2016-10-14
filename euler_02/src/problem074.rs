//! [74] Digit factorial chains
//! ---------------------------
//!
use std::collections::HashSet;

struct Digit {
    curr: usize,
}

impl Iterator for Digit {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 0 {
            return None;
        }
        let d = self.curr % 10;
        self.curr /= 10;
        Some(d)
    }
}

fn digits(n: usize) -> Digit {
    Digit { curr: n }
}

const TABLE: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn factorial_sum(n: usize) -> usize {
    digits(n).map(|d| TABLE[d]).sum()
}

fn chain_count(mut n: usize) -> usize {
    let mut set = HashSet::new();
    loop {
        if set.contains(&n) {
            break;
        }
        set.insert(n);
        n = factorial_sum(n);
    }
    set.len()
}

pub fn solve() -> u64 {
    let mut count = 0;
    for n in 1..1_000_000 {
        if chain_count(n) == 60 {
            count += 1;
        }
    }
    count
}
