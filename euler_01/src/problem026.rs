//! [26] Reciprocal cycles
//! ----------------------
//!
//! https://projecteuler.net/problem=26
//!
use std::collections::HashMap;

struct Fraction {
    num: usize,
    denom: usize,
}

impl Iterator for Fraction {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            None
        } else {
            let dividend = self.num * 10;
            let quotient = dividend / self.denom;
            self.num = dividend % self.denom;
            Some((dividend, quotient))
        }
    }
}

fn fractions(denom: usize) -> Fraction {
    Fraction {
        num: 1,
        denom: denom,
    }
}

pub fn solve() -> u64 {
    (2..1000).map(|n| (recurring_cycle_len(n), n)).max().unwrap().1 as u64
}

fn recurring_cycle_len(n: usize) -> usize {
    let mut map = HashMap::new();
    (1..)
        .zip(fractions(n))
        .map(|(i, dividend_quotient)| {
            match map.insert(dividend_quotient, i) {
                None => 0,
                Some(j) => i - j,
            }
        })
        .find(|&x| x != 0)
        .unwrap_or(0)
}
