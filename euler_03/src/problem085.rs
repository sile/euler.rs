//! [85] Counting rectangles
//!
//!
use std::collections::HashSet;

pub fn solve() -> u64 {
    let target = 2_000_000;
    let mut nearest = ((0, 0), target);
    fn add(acc: &mut usize, x: usize) -> Option<usize> {
        *acc += x;
        Some(*acc)
    }
    for (width, x_acc) in (0..(target + 1)).scan(0, add).enumerate().skip(1) {
        for (height, y_acc) in (0..(width + 1)).scan(0, add).enumerate().skip(1) {
            let count = x_acc * y_acc;
            let delta = delta(target, count);
            if delta < nearest.1 {
                nearest = ((width, height), delta);
            }
            if count >= target {
                break;
            }
        }
    }
    assert_eq!(nearest.1,
               delta(target,
                     count_rectangles((nearest.0).0, (nearest.0).1, &mut HashSet::new())));
    let area = (nearest.0).0 * (nearest.0).1;
    area as u64
}

fn delta(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

fn count_rectangles(x: usize, y: usize, known: &mut HashSet<(usize, usize)>) -> usize {
    if x == 0 || y == 0 || known.contains(&(x, y)) {
        return 0;
    }
    known.insert((x, y));

    (x * y) + count_rectangles(x - 1, y, known) + count_rectangles(x, y - 1, known)
}
