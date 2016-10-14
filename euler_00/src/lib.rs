extern crate num;
extern crate rand;
extern crate euler_lib;

mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;
mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;
mod problem018;
mod problem019;
mod problem020;
mod problem021;
mod problem022;
mod problem023;
mod problem024;
mod problem025;

pub fn solve(problem: usize) -> Option<u64> {
    [problem001::solve,
     problem002::solve,
     problem003::solve,
     problem004::solve,
     problem005::solve,
     problem006::solve,
     problem007::solve,
     problem008::solve,
     problem009::solve,
     problem010::solve,
     problem011::solve,
     problem012::solve,
     problem013::solve,
     problem014::solve,
     problem015::solve,
     problem016::solve,
     problem017::solve,
     problem018::solve,
     problem019::solve,
     problem020::solve,
     problem021::solve,
     problem022::solve,
     problem023::solve,
     problem024::solve,
     problem025::solve]
        .get(problem - 1)
        .map(|solve| solve())
}
