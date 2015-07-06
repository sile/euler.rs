//! [42] Coded triangle numbers
//! ---------------------------
//!
//! https://projecteuler.net/problem=42
//!
use utils::{self,Sum};
use std::collections::HashSet;
use std::iter::FromIterator;

struct Triangle {
    curr: usize,
}

impl Iterator for Triangle {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let n = self.curr;
        self.curr += 1;
        Some(n * (n+1) / 2)
    }
}

fn triangle_numbers() -> Triangle{ Triangle{curr: 1} }

pub fn solve() -> usize {
    let words = utils::load_names("data/p042_names.txt");
    let values: Vec<_> = words.iter().map(|name| word_value(&name) ).collect();
    let &max = values.iter().max().unwrap();
    let triangles: HashSet<_> = HashSet::from_iter(triangle_numbers().take_while(|&n| n <= max ));
    values.iter().filter(|n| triangles.contains(n) ).count()
}

fn word_value(word: &str) -> usize {
    word.bytes().map(|b| b as usize - 64 ).summation()
}
