extern crate num;
extern crate rand;
extern crate euler_lib;

mod problem051;
mod problem052;
mod problem053;
mod problem054;
mod problem055;
mod problem056;
mod problem057;
mod problem058;
mod problem059;
mod problem060;
mod problem061;
mod problem062;
mod problem063;
mod problem064;
mod problem065;
mod problem066;
mod problem067;
mod problem068;
mod problem069;
mod problem070;
mod problem071;
mod problem072;
mod problem073;
mod problem074;
mod problem075;

pub fn solve(problem: usize) -> Option<u64> {
    [problem051::solve,
     problem052::solve,
     problem053::solve,
     problem054::solve,
     problem055::solve,
     problem056::solve,
     problem057::solve,
     problem058::solve,
     problem059::solve,
     problem060::solve,
     problem061::solve,
     problem062::solve,
     problem063::solve,
     problem064::solve,
     problem065::solve,
     problem066::solve,
     problem067::solve,
     problem068::solve,
     problem069::solve,
     problem070::solve,
     problem071::solve,
     problem072::solve,
     problem073::solve,
     problem074::solve,
     problem075::solve]
        .get(problem - 51)
        .map(|solve| solve())
}
