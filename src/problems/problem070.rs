//! [70] Totient permutation
//! ------------------------
//!
extern crate num;

use utils;
use self::num::rational::Ratio;

const LIMIT: usize = 10_000_000;

pub fn solve() -> usize {
    let mut table = vec![Ratio::new(1,1); LIMIT];
    for p in utils::primes().map(|p| p as usize).take_while(|&p| p < LIMIT) {
        let r = Ratio::new(p, p - 1);
        for i in (1..LIMIT/p+1).map(|i| i * p ) {
            if i >= table.len() { continue }
            table[i] = table[i] * r.clone();
        }
    }
    table.into_iter()
        .zip(0..)
        .skip(2)
        .filter(|&(r, n)| is_permutation(n, totient(n, &r)) )
        .min().unwrap().1
}

fn totient(n: usize, r: &Ratio<usize>) -> usize {
    let t = Ratio::new(*r.denom() * n, *r.numer());
    assert!(t.is_integer());
    t.to_integer()
}

fn is_permutation(mut x: usize, mut y: usize) -> bool {
    let mut count: [isize; 10] = [0; 10];
    while x > 0 {
        count[x % 10] += 1;
        x /= 10;
    }
    while y > 0 {
        count[y % 10] -= 1;
        y /= 10;
    }
    count.into_iter().all(|&c| c == 0 )
}
