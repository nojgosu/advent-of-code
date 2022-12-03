use crate::packet_decoder::solve::{solve_first_star, solve_second_star};

mod solve;

pub fn packet_decoder_solution() {
    println!("--- Day 16: Packet Decoder ---");
    let result = solve_first_star();
    println!("\t1st Star Solution = {}", result);

    let result = solve_second_star();
    println!("\t2nd Star Solution = {}", result);
}