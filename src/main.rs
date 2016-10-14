extern crate num;
extern crate euler_00;
extern crate euler_01;
extern crate euler_02;
extern crate euler_03;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} PROBLEM_NUMBER", args[0]);
    } else {
        let n: usize = args[1].parse().unwrap();
        let answer = match n {
            1...25 => euler_00::solve(n),
            26...50 => euler_01::solve(n),
            51...75 => euler_02::solve(n),
            76...100 => euler_03::solve(n),
            _ => None,
        };
        if let Some(a) = answer {
            println!("[{}] answer: {}", n, a);
        } else {
            println!("{} is an unsolved problem", n);
        }
    }
}
