//! [42] Coded triangle numbers
//! ---------------------------
//!
//! https://projecteuler.net/problem=42
//!
use utils::{self, Sum};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() -> usize {
    let words = utils::load_names("data/p042_names.txt");
    let values: Vec<_> = words.iter().map(|name| word_value(&name)).collect();
    let &max = values.iter().max().unwrap();
    let triangles: HashSet<_> = HashSet::from_iter(utils::triangle_numbers(1)
                                                       .take_while(|&n| n <= max));
    values.iter().filter(|n| triangles.contains(n)).count()
}

fn word_value(word: &str) -> u64 {
    word.bytes().map(|b| b as u64 - 64).summation()
}
