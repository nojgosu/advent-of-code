use crate::giant_squid::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn giant_squid_solution() {
    println!("--- Day 4: Giant Squid ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}