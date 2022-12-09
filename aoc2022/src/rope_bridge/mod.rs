use crate::rope_bridge::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn rope_bridge_solution() {
    println!("--- Day 9: Rope Bridge ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}