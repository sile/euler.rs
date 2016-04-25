//! [22] Names scores
//! -----------------
//!
//! ### Problem
//! ```
//! Using [names.txt](https://projecteuler.net/project/resources/p022_names.txt) (right click and 'Save Link/Target As...'),
//! a 46K text file containing over five-thousand first names, begin by sorting it
//! into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in
//! the list to obtain a name score.
//!
//! For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in
//! the list. So, COLIN would obtain a score of 938 × 53 = 49714.
//!
//! What is the total of all the name scores in the file?
//! ```
use utils::{self, Sum};

pub fn solve() -> usize {
    let mut names = utils::load_names("data/p022_names.txt");
    names.sort_by(|a, b| a.cmp(b));
    names.iter().zip(1..).map(|(name, i)| score(i, name)).summation()
}

fn score(rank: usize, name: &str) -> usize {
    name.bytes().map(|c| c as usize - 64).summation() * rank
}
