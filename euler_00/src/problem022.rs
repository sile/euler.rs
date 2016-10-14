//! [22] Names scores
//! -----------------
//!
use euler_lib::utils;

pub fn solve() -> u64 {
    let mut names = utils::load_names("data/p022_names.txt");
    names.sort_by(|a, b| a.cmp(b));
    names.iter().zip(1..).map(|(name, i)| score(i, name) as u64).sum()
}

fn score(rank: usize, name: &str) -> usize {
    name.bytes().map(|c| c as usize - 64).sum::<usize>() * rank
}
