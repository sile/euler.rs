extern crate num;
extern crate rand;
extern crate euler_lib;

mod problem076;
mod problem077;
mod problem078;
mod problem079;
mod problem080;
mod problem081;
mod problem082;
mod problem083;
mod problem084;
mod problem085;
mod problem086;
mod problem087;
mod problem088;

pub fn solve(problem: usize) -> Option<u64> {
    [problem076::solve,
     problem077::solve,
     problem078::solve,
     problem079::solve,
     problem080::solve,
     problem081::solve,
     problem082::solve,
     problem083::solve,
     problem084::solve,
     problem085::solve,
     problem086::solve,
     problem087::solve,
     problem088::solve]
        .get(problem - 76)
        .map(|solve| solve())
}
