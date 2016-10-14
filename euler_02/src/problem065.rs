//! [65] Convergents of e
//! ---------------------
//!
use num::rational::Ratio;
use num::bigint::BigUint;
use num::{One, FromPrimitive};

pub fn solve() -> u64 {
    let nth = 100;
    let fractions: Vec<_> = vec![2, 1]
        .into_iter()
        .chain((1..)
            .map(|i| 2 * i)
            .flat_map(|x| vec![x, 1, 1].into_iter()))
        .take(nth)
        .collect();

    let mut convergent = Ratio::from_integer(BigUint::from_usize(fractions[nth - 1]).unwrap());
    for i in 0..nth - 1 {
        let n = fractions[nth - i - 2];
        convergent = Ratio::from_integer(BigUint::one()) / convergent;
        convergent = Ratio::from_integer(BigUint::from_usize(n).unwrap()) + convergent;
    }
    format!("{}", convergent.numer()).chars().map(|c| c.to_digit(10).unwrap() as u64).sum()
}
