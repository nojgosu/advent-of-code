use crate::seven_segment_search::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn seven_segment_search_solution() {
    println!("--- Day 8: Seven Segment Search ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}