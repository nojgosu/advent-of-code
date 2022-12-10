use crate::cathode_ray_tube::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn cathode_ray_tube_solution() {
    println!("--- Day 10: Cathode-Ray Tube ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}