use crate::beacon_exclusion_zone::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn beacon_exclusion_zone_solution() {
    println!("--- Day 15: Beacon Exclusion Zone ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}