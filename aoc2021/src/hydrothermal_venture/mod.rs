use crate::hydrothermal_venture::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn hydrothermal_venture_solution() {
    println!("--- Day 5: Hydrothermal Venture ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}