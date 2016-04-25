//! [44] Pentagon numbers
//! ---------------------
//!
//! https://projecteuler.net/problem=44
//!
use std::collections::HashSet;
use std::cmp;
use utils;

pub fn solve() -> u64 {
    let mut min = u64::max_value();
    let mut last = 1;
    let mut pentagonals = HashSet::new();
    let mut start = 1;

    for x in utils::pentagonal_numbers(2) {
        if x - last >= min {
            break;
        }
        for y in utils::pentagonal_numbers(pentagonals.len() as u64 + 1)
                     .take_while(|&y| y <= x + last) {
            pentagonals.insert(y);
        }

        min = cmp::min(min,
                       utils::pentagonal_numbers(start)
                           .take_while(|&y| y < x)
                           .filter(|&y| {
                               pentagonals.contains(&(x - y)) && pentagonals.contains(&(x + y))
                           })
                           .map(|y| x - y)
                           .last()
                           .unwrap_or(u64::max_value()));
        start += utils::pentagonal_numbers(start).take_while(|&y| x - y >= min).count() as u64;
        last = x;
    }
    min
}
