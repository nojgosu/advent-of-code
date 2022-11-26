use crate::smoke_basin::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn smoke_basin_solution() {
    println!("--- Day 9: Smoke Basin ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}