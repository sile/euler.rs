//! [69] Totient maximum
//! ---------------------
//!
extern crate num;

use utils;
use self::num::rational::Ratio;

const LIMIT: usize = 1_000_001;

pub fn solve() -> usize {
    let mut table = vec![Ratio::new(1,1); LIMIT];
    for p in utils::primes().map(|p| p as usize).take_while(|&p| p < LIMIT) {
        let r = Ratio::new(p, p - 1);
        for i in (1..LIMIT/p).map(|i| i * p ) {
            table[i] = table[i] * r.clone();
        }
    }
    table.into_iter().zip(0..).max().unwrap().1
}
