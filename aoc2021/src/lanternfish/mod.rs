use crate::lanternfish::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn lanternfish_solution() {
    println!("--- Day 6: Lanternfish ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}