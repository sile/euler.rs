//! [67] Maximum path sum II
//! -------------------------
//!
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::cmp;

pub fn solve() -> usize {
    let triangle = load_triangle();
    let mut table = vec![0; triangle.len()];

    let mut row = 0;
    let mut col = 0;
    for i in 0..triangle.len() {
        let mut parent_l = 0;
        let mut parent_r = 0;
        if col > 0   { parent_l = table[i - row - 1]; }
        if col < row { parent_r = table[i - row];     }

        table[i] = triangle[i] + cmp::max(parent_l, parent_r);

        if col == row {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    table.into_iter().max().unwrap()
}

fn load_triangle() -> Vec<usize> {
    let mut nums = Vec::new();
    let mut f = File::open(Path::new("data/p067_triangle.txt")).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();
    for line in s.split('\n') {
        if line.len() == 0 { continue }
        for n in line.split(' ').map(|n| usize::from_str_radix(n, 10).unwrap() ) {
            nums.push(n);
        }
    }
    nums
}
