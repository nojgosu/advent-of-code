use crate::transparent_origami::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn transparent_origami_solution() {
    println!("--- Day 13: Transparent Origami ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}