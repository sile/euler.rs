//! [40] Champernowne's constant
//! ----------------------------
//!
//! https://projecteuler.net/problem=40
//!
use utils::{self,Product};

struct IrrationalDecimalFraction {
    next: usize,
    curr: Vec<usize>,
}

impl Iterator for IrrationalDecimalFraction {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.curr.len() == 0 {
            self.curr = utils::int_to_digits(self.next);
            self.next += 1;
        }
        let d = self.curr.pop().unwrap();
        Some(d)
    }
}

fn irrational_decimal_fraction() -> IrrationalDecimalFraction {
    IrrationalDecimalFraction{next: 1, curr: Vec::new()}
}

pub fn solve() -> usize {
    (1..).zip(irrational_decimal_fraction())
        .filter(|&(i, _)| match i { 1 | 10 | 100 | 1000 | 10000 | 100000 | 1000000 => true, _ => false} )
        .map(|(_, d)| d )
        .take(7)
        .prod()
}