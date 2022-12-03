use crate::calorie_counting::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn calorie_counting_solution() {
    println!("--- Day 1: Calorie Counting ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}