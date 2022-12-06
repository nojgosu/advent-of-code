use crate::tuning_trouble::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn tuning_trouble_solution() {
    println!("--- Day 6: Tuning Trouble ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}