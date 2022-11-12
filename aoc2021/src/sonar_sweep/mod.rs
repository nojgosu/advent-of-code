use crate::sonar_sweep::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn sonar_sweep_solution() -> () {
    let result = solve_first_star();
    println!("Sonar Sweep 1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("Sonar Sweep 2nd Star Solution = {}", result);
}