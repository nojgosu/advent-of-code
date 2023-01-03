use crate::proboscidea_volcanium::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn proboscidea_volcanium_solution() {
    println!("--- Day 16: Proboscidea Volcanium Zone ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}