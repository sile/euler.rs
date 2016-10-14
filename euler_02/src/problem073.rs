//! [73] Counting fractions in a range
//! ----------------------------------
//!
use num::integer;
use num::rational::Ratio;

pub fn solve() -> u64 {
    let limit = 12_001;
    let lower = Ratio::new(1, 3);
    let upper = Ratio::new(1, 2);
    let mut count = 0;
    for d in 2..limit {
        let n_min = (Ratio::from_integer(d) * lower).ceil().to_integer();
        let n_max = (Ratio::from_integer(d) * upper).ceil().to_integer();
        for n in n_min..n_max {
            let r = Ratio::new(n, d);
            if integer::gcd(n, d) == 1 && lower < r && r < upper {
                count += 1;
            }
        }
    }
    count as u64
}
