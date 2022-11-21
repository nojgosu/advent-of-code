use crate::treachery_of_whales::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn treachery_of_whales_solution() {
    println!("--- Day 7: The Treachery of Whales ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}