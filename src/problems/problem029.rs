//! [29] Distinct powers
//! --------------------
//!
//! https://projecteuler.net/problem=29
//!
extern crate num;

use std::collections::HashSet;
use self::num::bigint::BigUint;
use self::num::FromPrimitive;

pub fn solve() -> usize {
    let n = 100;
    let mut v = HashSet::new();
    for a in (2..n+1) {
        for b in (2..n+1) {
            let x = num::pow(BigUint::from_usize(a).unwrap(), b);
            v.insert(x);
        }
    }
    v.len()
}
