//! [45] Triangular, pentagonal, and hexagonal
//! ------------------------------------------
//!
//! https://projecteuler.net/problem=45
//!
use utils;
use std::iter::Peekable;

pub fn solve() -> u64 {
    let mut pentagonal = utils::pentagonal_numbers(1).peekable();
    let mut hexagonal = utils::hexagonal_numbers(1).peekable();
    utils::triangle_numbers(1)
        .map(|x| {
            let y = next_not_less_than(&mut pentagonal, x);
            let z = next_not_less_than(&mut hexagonal, x);
            (x, y, z)
        })
        .filter(|&(x, y, z)| x == y && y == z )
        .nth(2).unwrap().0
}

fn next_not_less_than<I: Iterator<Item=u64>>(itr: &mut Peekable<I>, n: u64) -> u64 {
    let mut r = 0;
    while let Some(&m) = itr.peek() {
        if m >= n {
            r = m;
            break;
        }
        itr.next();
    }
    r
}
