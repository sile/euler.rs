//! [71] Ordered fractions
//! ----------------------
//!
use num::integer;
use num::rational::Ratio;

pub fn solve() -> usize {
    let limit = 1_000_001;
    let max = Ratio::new(3, 7);
    let mut min = Ratio::new(1, limit);
    for d in 2..limit {
        let n_min = (Ratio::from_integer(d) * min).floor().to_integer();
        let n_max = (Ratio::from_integer(d) * max).ceil().to_integer();
        for n in n_min..n_max + 1 {
            if integer::gcd(n, d) == 1 {
                let cur = Ratio::new(n, d);
                if min < cur && cur < max {
                    min = cur;
                    println!("    new immediate left: {:?}", min);
                }
            }
        }
    }
    *min.numer()
}
