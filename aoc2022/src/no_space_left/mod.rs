use crate::no_space_left::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn no_space_left_solution() {
    println!("--- Day 7: No Space Left On Device ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}