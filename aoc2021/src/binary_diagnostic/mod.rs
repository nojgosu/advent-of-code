use crate::binary_diagnostic::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn binary_diagnostic_solution() -> () {
    let result = solve_first_star();
    println!("Binary Diagnostic 1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("Binary Diagnostic 2nd Star Solution = {}", result);
}