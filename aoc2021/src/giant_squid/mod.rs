use crate::giant_squid::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn giant_squid_solution() {
    let result = solve_first_star();
    println!("Giant Squid 1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("Giant Squid 2nd Star Solution = {}", result);
}