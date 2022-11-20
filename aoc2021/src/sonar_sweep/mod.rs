use crate::sonar_sweep::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn sonar_sweep_solution() {
    println!("--- Day 1: Sonar Sweep ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}