use crate::hill_climbing::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn hill_climbing_solution() {
    println!("--- Day 12: Hill Climbing Algorithm ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}