use crate::rock_paper_scissors::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn rock_paper_scissors_solution() {
    println!("--- Day 2: Rock Paper Scissors ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}