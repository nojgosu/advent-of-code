use crate::syntax_scoring::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn syntax_scoring_solution() {
    println!("--- Day 10: Syntax Scoring ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}