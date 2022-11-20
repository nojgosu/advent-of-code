use crate::binary_diagnostic::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn binary_diagnostic_solution() {
    println!("--- Day 3: Binary Diagnostic ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}