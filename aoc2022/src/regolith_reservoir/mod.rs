use crate::regolith_reservoir::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn regolith_reservoir_solution() {
    println!("--- Day 14: Regolith Reservoir ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}