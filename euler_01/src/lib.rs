extern crate num;
extern crate rand;
extern crate euler_lib;

mod problem026;
mod problem027;
mod problem028;
mod problem029;
mod problem030;
mod problem031;
mod problem032;
mod problem033;
mod problem034;
mod problem035;
mod problem036;
mod problem037;
mod problem038;
mod problem039;
mod problem040;
mod problem041;
mod problem042;
mod problem043;
mod problem044;
mod problem045;
mod problem046;
mod problem047;
mod problem048;
mod problem049;
mod problem050;

pub fn solve(problem: usize) -> Option<u64> {
    [problem026::solve,
     problem027::solve,
     problem028::solve,
     problem029::solve,
     problem030::solve,
     problem031::solve,
     problem032::solve,
     problem033::solve,
     problem034::solve,
     problem035::solve,
     problem036::solve,
     problem037::solve,
     problem038::solve,
     problem039::solve,
     problem040::solve,
     problem041::solve,
     problem042::solve,
     problem043::solve,
     problem044::solve,
     problem045::solve,
     problem046::solve,
     problem047::solve,
     problem048::solve,
     problem049::solve,
     problem050::solve]
        .get(problem - 26)
        .map(|solve| solve())
}
