//! [34] Digit factorials
//! -------------------------------
//!
//! https://projecteuler.net/problem=34
//!
use utils::{self, Sum};

pub fn solve() -> usize {
    let factorial_of_9: usize = utils::factorial(9);

    (3..)
        .map(|n| (n, format!("{}", n)))
        .take_while(|n| n.0 <= factorial_of_9 * n.1.len())
        .filter(|n| {
            let x: usize = n.1
                            .chars()
                            .map(|c| c.to_digit(10).unwrap() as usize)
                            .map(|x| utils::factorial(x))
                            .summation();
            x == n.0
        })
        .map(|n| n.0)
        .summation()
}
