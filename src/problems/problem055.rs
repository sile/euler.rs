//! [55] Lychrel numbers
//! --------------------
//!
//! https://projecteuler.net/problem=55
//!
extern crate num;
use self::num::FromPrimitive;
use self::num::bigint::BigUint;

pub fn solve() -> usize {
    (1..10_000).filter(|&n| is_lychrel_number(n) ).count()
}

fn is_lychrel_number(n: u64) -> bool {
     let mut n = BigUint::from_u64(n).unwrap();
    n = n.clone() + reverse(n);
    for _ in 0..49 {
        let r = reverse(n.clone());
        if n == r {
            return false;
        }
        n = n + r;
    }
    true
}

fn reverse(mut n: BigUint) -> BigUint {
    let mut r = num::zero();
    let ten = BigUint::from_u64(10).unwrap();
    while n != num::zero() {
        r = r * ten.clone();
        r = r + n.clone() % ten.clone();
        n = n / ten.clone();
    }
    r
}
