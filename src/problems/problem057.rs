//! [57] Square root convergents
//! ----------------------------
//!
//! https://projecteuler.net/problem=57
//!
use std::ops::Add;
use num::bigint::BigUint;
use num::FromPrimitive;

struct Convergent {
    num: BigUint,
    denom: BigUint,
}

impl Iterator for Convergent {
    type Item = (BigUint, BigUint);
    fn next(&mut self) -> Option<Self::Item> {
        let mut num = self.num.clone().add(&self.denom);
        let curr = (num.clone(), self.denom.clone());

        num = num.add(&self.denom);
        self.num.clone_from(&self.denom);
        self.denom.clone_from(&num);

        Some(curr)
    }
}

fn square_root_convergents() -> Convergent {
    Convergent{num: BigUint::from_u8(1).unwrap(), denom: BigUint::from_u8(2).unwrap()}
}

pub fn solve() -> usize {
    square_root_convergents().take(1000).filter(|x| digits(&x.0) > digits(&x.1) ).count()
}

fn digits(n: &BigUint) -> usize {
    format!("{}", n).len()
}
