//! [63] Powerful digit counts
//! --------------------------
//!
extern crate num;

use self::num::bigint::BigUint;
use self::num::FromPrimitive;
use utils::Sum;

pub fn solve() -> usize {
    (1..)
        .map(|x| {
            (1..).map(|y| format!("{}", num::pow(BigUint::from_usize(y).unwrap(), x)).len() ).take_while(|&d| d <= x ).filter(|&d| d == x).count()
        })
        .take_while(|&c| c != 0 )
        .summation()
}
