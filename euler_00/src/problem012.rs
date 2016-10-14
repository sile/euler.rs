//! [12] Highly divisible triangular number
//! ---------------------------------------
//!
pub fn solve() -> u64 {
    let divisors = 500;
    let mut triangle_num = 0;
    for n in 1.. {
        triangle_num += n;
        if (1..)
            .take_while(|x| x * x <= triangle_num)
            .filter(|x| triangle_num % x == 0)
            .map(|x| { if x * x == triangle_num { 1 } else { 2 } })
            .sum::<u64>() >= divisors {
            break;
        }
    }
    triangle_num
}
