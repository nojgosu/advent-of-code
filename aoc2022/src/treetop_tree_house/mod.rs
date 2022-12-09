use crate::treetop_tree_house::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn treetop_tree_house_solution() {
    println!("--- Day 8: Treetop Tree House ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}