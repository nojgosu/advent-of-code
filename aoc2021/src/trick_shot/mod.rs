use crate::trick_shot::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn trick_shot_solution() {
    println!("--- Day 17: Trick Shot ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}