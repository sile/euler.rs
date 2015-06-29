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
            1   => println!("[{}] answer: {:?}", n, problem001::solve()),
            2   => println!("[{}] answer: {:?}", n, problem002::solve()),
            3   => println!("[{}] answer: {:?}", n, problem003::solve()),
            4   => println!("[{}] answer: {:?}", n, problem004::solve()),
            5   => println!("[{}] answer: {:?}", n, problem005::solve()),
            6   => println!("[{}] answer: {:?}", n, problem006::solve()),
            7   => println!("[{}] answer: {:?}", n, problem007::solve()),
            8   => println!("[{}] answer: {:?}", n, problem008::solve()),
            9   => println!("[{}] answer: {:?}", n, problem009::solve()),
            10  => println!("[{}] answer: {:?}", n, problem010::solve()),
            11  => println!("[{}] answer: {:?}", n, problem011::solve()),
            12  => println!("[{}] answer: {:?}", n, problem012::solve()),
            13  => println!("[{}] answer: {:?}", n, problem013::solve()),
            14  => println!("[{}] answer: {:?}", n, problem014::solve()),
            15  => println!("[{}] answer: {:?}", n, problem015::solve()),
            16  => println!("[{}] answer: {:?}", n, problem016::solve()),
            17  => println!("[{}] answer: {:?}", n, problem017::solve()),
            18  => println!("[{}] answer: {:?}", n, problem018::solve()),
            19  => println!("[{}] answer: {:?}", n, problem019::solve()),
            20  => println!("[{}] answer: {:?}", n, problem020::solve()),
            21  => println!("[{}] answer: {:?}", n, problem021::solve()),
            22  => println!("[{}] answer: {:?}", n, problem022::solve()),
            23  => println!("[{}] answer: {:?}", n, problem023::solve()),
            24  => println!("[{}] answer: {:?}", n, problem024::solve()),
            25  => println!("[{}] answer: {:?}", n, problem025::solve()),
            26  => println!("[{}] answer: {:?}", n, problem026::solve()),
            27  => println!("[{}] answer: {:?}", n, problem027::solve()),
            _   => println!("{} is an unsolved problem", n),
        }
    }
}
