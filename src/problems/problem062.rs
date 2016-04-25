//! [62] Cubic permutations
//! -----------------------
//!
use std::collections::HashMap;

pub fn solve() -> u64 {
    let mut cubes = HashMap::new();
    for n in 1.. {
        let cube = cube(n);
        let key = digit_rsort(cube);
        if !cubes.contains_key(&key) {
            cubes.insert(key, vec![cube]);
        } else {
            cubes.get_mut(&key).unwrap().push(cube);
            if cubes.get(&key).unwrap().len() == 5 {
                return cubes.get(&key).unwrap()[0];
            }
        }
    }
    unreachable!()
}

fn cube(n: u64) -> u64 {
    n * n * n
}

fn digit_rsort(n: u64) -> String {
    let mut bs: Vec<_> = format!("{}", n).bytes().collect();
    bs.sort();
    bs.reverse();
    String::from_utf8(bs).unwrap()
}
