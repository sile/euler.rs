//! [63] Powerful digit counts
//! --------------------------
//!
use num;
use num::bigint::BigUint;
use num::FromPrimitive;

pub fn solve() -> u64 {
    (1..)
        .map(|x| {
            (1..)
                .map(|y| format!("{}", num::pow(BigUint::from_usize(y).unwrap(), x)).len())
                .take_while(|&d| d <= x)
                .filter(|&d| d == x)
                .count()
        })
        .take_while(|&c| c != 0)
        .map(|x| x as u64)
        .sum()
}
