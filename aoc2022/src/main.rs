
use crate::calorie_counting::calorie_counting_solution;
use crate::rock_paper_scissors::rock_paper_scissors_solution;
use crate::rucksack_reorganisation::rucksack_reorganisation_solution;

mod calorie_counting;
mod rock_paper_scissors;
mod rucksack_reorganisation;


fn main() {
    println!("*** Advent of Code 2022 ***");
    calorie_counting_solution();
    rock_paper_scissors_solution();
    rucksack_reorganisation_solution();

}