use crate::distress_signal::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn distress_signal_solution() {
    println!("--- Day 13: Distress Signal ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}