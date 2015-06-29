//! [31] Coin sums
//! -----------------------
//!
//! https://projecteuler.net/problem=31
//!
pub fn solve() -> usize {
    let coin: [usize; 8] = [200,100,50,20,10,5,2,1];
    pattern_count(200, &coin)
}

fn pattern_count(amount: usize, coin: &[usize]) -> usize {
    if coin.len() == 0 { return 0; }
    if amount == 0 { return 1; }

    let mut n = 0;
    if amount >= coin[0] {
        n += pattern_count(amount - coin[0], coin);
    }
    n += pattern_count(amount, &coin[1..]);
    n
}
