use crate::day4::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn day4_solution() {
    println!("--- Day 4: Scratchcards ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}