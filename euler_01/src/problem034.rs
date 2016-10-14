//! [34] Digit factorials
//! -------------------------------
//!
//! https://projecteuler.net/problem=34
//!
use euler_lib::utils;

pub fn solve() -> u64 {
    let factorial_of_9: u64 = utils::factorial(9);

    (3..)
        .map(|n| (n, format!("{}", n)))
        .take_while(|n| n.0 <= factorial_of_9 * n.1.len() as u64)
        .filter(|n| {
            let x: u64 = n.1
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|x| utils::factorial::<u64>(x as usize))
                .sum();
            x == n.0
        })
        .map(|n| n.0)
        .sum()
}
