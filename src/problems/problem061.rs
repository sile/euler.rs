//! [61] Cyclical figurate numbers
//! ------------------------------
//!
use utils::{self, Sum};
use std::collections::{HashMap, HashSet};

type Edges = HashMap<usize, Vec<(usize, usize)>>;

pub fn solve() -> usize {
    let mut edges = HashMap::new();

    // build edges
    for i in 3..9 {
        for n in 1.. {
            let p = polygonal(i, n);
            let d = utils::digits(p);
            if d > 4 {
                break;
            }
            if d == 4 {
                let high = p / 100;
                let low = p % 100;
                if low <= 9 {
                    continue;
                }
                if !edges.contains_key(&high) {
                    edges.insert(high, vec![]);
                }
                edges.get_mut(&high).unwrap().push((i, low));
            }
        }
    }

    // find cyclic
    edges.keys()
         .filter_map(|&high| {
             find_cyclic(high, high, &edges, &mut HashSet::new(), &mut HashSet::new())
         })
         .nth(0)
         .unwrap()
         .into_iter()
         .summation()
}

fn find_cyclic(first: usize,
               high: usize,
               edges: &Edges,
               polys: &mut HashSet<usize>,
               nums: &mut HashSet<usize>)
               -> Option<HashSet<usize>> {
    if nums.len() == 6 && first == high {
        return Some(nums.clone());
    }
    match edges.get(&high) {
        None => None,
        Some(lows) => {
            lows.iter()
                .filter_map(|&(poly, low)| {
                    if polys.contains(&poly) {
                        return None;
                    }
                    let num = high * 100 + low;
                    if nums.contains(&num) {
                        return None;
                    }
                    polys.insert(poly);
                    nums.insert(num);
                    let ret = find_cyclic(first, low, edges, polys, nums);
                    polys.remove(&poly);
                    nums.remove(&num);
                    ret
                })
                .nth(0)
        }
    }
}

fn polygonal(i: usize, n: usize) -> usize {
    match i {
        3 => n * (n + 1) / 2,     // triangle
        4 => n * n,               // square
        5 => n * (3 * n - 1) / 2, // pentagonal
        6 => n * (2 * n - 1),     // hexagonal
        7 => n * (5 * n - 3) / 2, // heptagonal
        8 => n * (3 * n - 2),     // octagonal
        _ => unreachable!(),
    }
}
