use crate::monkey_in_the_middle::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn monkey_in_the_middle_solution() {
    println!("--- Day 11: Monkey In The Middle ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}