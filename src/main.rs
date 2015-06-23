use std::env;

pub mod utils;
pub mod problems;
use problems::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} PROBLEM_NUMBER", args[0]);
    } else {
        let n: usize = args[1].parse().unwrap();
        match n {
            1 => println!("[{}] answer: {:?}", n, problem001::solve()),
            2 => println!("[{}] answer: {:?}", n, problem002::solve()),
            3 => println!("[{}] answer: {:?}", n, problem003::solve()),
            4 => println!("[{}] answer: {:?}", n, problem004::solve()),
            5 => println!("[{}] answer: {:?}", n, problem005::solve()),
            6 => println!("[{}] answer: {:?}", n, problem006::solve()),
            7 => println!("[{}] answer: {:?}", n, problem007::solve()),
            _ => println!("{} is an unsolved problem", n),
        }
    }
}
