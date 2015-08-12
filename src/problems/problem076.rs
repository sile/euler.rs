//! [76] Counting summations
//! ------------------------
//!
pub fn solve() -> usize {
    let max = 100;
    let mut ways_table: Vec<Vec<_>> = Vec::with_capacity(max+1);
    for n in 0..max+1 {
        let mut ways = vec![0; n+1];
        for lft in 1..n/2+1 {
            let rgt = n - lft;
            ways[lft] = ways_table[rgt][lft..].iter().fold(0, |x,y| x+y) + 1;
        }
        ways_table.push(ways);
    }
    ways_table[max].iter().fold(0, |x,y| x+y )
}
