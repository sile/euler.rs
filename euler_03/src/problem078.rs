//! [78] Coin partitions
//! ---------------------
//!
//! cf. problem076
pub fn solve() -> u64 {
    let num: u64 = 1_000_000;
    let mut ways_table: Vec<Vec<_>> = Vec::new();
    (0..)
        .find(|&n| {
            let mut ways = vec![0; n/2+1];
            let mut sum = 0;
            for i in 1..n / 2 + 1 {
                let lft = n / 2 + 1 - i;
                let rgt = n - lft;
                if lft < ways_table[rgt].len() {
                    sum = (sum + ways_table[rgt][lft] + 1) % num;
                } else {
                    sum = (sum + 1) % num;
                }
                ways[lft] = sum;
            }
            ways_table.push(ways);
            println!("    {}: {}", n, sum + 1);
            sum + 1 == num
        })
        .unwrap() as u64
}
