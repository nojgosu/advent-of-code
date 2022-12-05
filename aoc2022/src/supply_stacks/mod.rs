use crate::supply_stacks::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn supply_stacks_solution() {
    println!("--- Day 5: Supply Stacks ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}