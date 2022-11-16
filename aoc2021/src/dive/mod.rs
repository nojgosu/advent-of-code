use crate::dive::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn dive_solution() {
    let result = solve_first_star();
    println!("Dive 1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("Dive 2nd Star Solution = {}", result);
}