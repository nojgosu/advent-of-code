use crate::rucksack_reorganisation::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn rucksack_reorganisation_solution() {
    println!("--- Day 3: Rucksack Reorganisation ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}