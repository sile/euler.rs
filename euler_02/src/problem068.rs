//! [68] Magic 5-gon ring
//! ---------------------
//!
use std::collections::HashSet;

pub fn solve() -> u64 {
    let mut ring = [0; 15];
    let mut results = Vec::new();
    fill_ring(&mut ring, 0, 0, &mut HashSet::new(), &mut results);
    *results.last().unwrap()
}

fn line_sum(ring: &[u64; 15], i: usize) -> u64 {
    ring[i] + ring[i + 1] + ring[i + 2]
}

fn fill_ring(ring: &mut [u64; 15],
             i: usize,
             first: u64,
             used: &mut HashSet<u64>,
             results: &mut Vec<u64>) {
    match i {
        15 => {
            if line_sum(ring, 0) == line_sum(ring, 3) && line_sum(ring, 3) == line_sum(ring, 9) &&
               line_sum(ring, 9) == line_sum(ring, 12) &&
               line_sum(ring, 12) == line_sum(ring, 0) {
                let s = ring.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().concat();
                let n = u64::from_str_radix(&s, 10).unwrap();
                println!("    {}", n);
                results.push(n);
            }
        }
        4 | 7 | 10 | 13 => {
            if ring[i - 2] == 10 {
                return;
            }
            ring[i] = ring[i - 2];
            fill_ring(ring, i + 1, first, used, results);
        }
        14 => {
            if ring[1] == 10 {
                return;
            }
            ring[i] = ring[1];
            fill_ring(ring, i + 1, first, used, results);
        }
        _ => {
            let start = match i {
                0 | 3 | 6 | 9 | 12 => first + 1,
                _ => 1,
            };
            for x in start..11 {
                if used.contains(&x) {
                    continue;
                }
                ring[i] = x;
                used.insert(x);
                fill_ring(ring,
                          i + 1,
                          (if i == 0 {
                              x
                          } else {
                              first
                          }),
                          used,
                          results);
                used.remove(&x);
            }
        }
    }
}
