//! [79] Passcode derivation
//! ------------------------
//!
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};
use utils;

pub fn solve() -> usize {
    let keylog = load("data/p079_keylog.txt");
    let mut precedings = HashMap::new();
    let mut passcode = Vec::new();

    for key in keylog.iter() {
        let digits = utils::int_to_digits(*key);
        for &d in digits.iter() {
            if precedings.get(&d).is_none() {
                precedings.insert(d, HashSet::new());
            }
        }
        precedings.get_mut(&digits[0]).unwrap().insert(digits[1]);
        precedings.get_mut(&digits[0]).unwrap().insert(digits[2]);
        precedings.get_mut(&digits[1]).unwrap().insert(digits[2]);
    }
    while precedings.len() > 0 {
        let head = *precedings.iter().find(|x| x.1.len() == 0).unwrap().0;
        precedings.remove(&head);
        for x in precedings.iter_mut() {
            x.1.remove(&head);
        }
        passcode.push(head);
    }

    utils::digits_to_int(&passcode)
}

fn load(filename: &str) -> Vec<usize> {
    let mut f = File::open(Path::new(filename)).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();
    s.split("\n").filter(|x| x.len() > 0).map(|x| usize::from_str_radix(x, 10).unwrap()).collect()
}
