use crate::calorie_counting::calorie_counting_solution;
use crate::rock_paper_scissors::rock_paper_scissors_solution;
use crate::rucksack_reorganisation::rucksack_reorganisation_solution;
use crate::camp_cleanup::camp_cleanup_solution;
use crate::supply_stacks::supply_stacks_solution;
use crate::tuning_trouble::tuning_trouble_solution;
use crate::no_space_left::no_space_left_solution;
use crate::treetop_tree_house::treetop_tree_house_solution;
use crate::rope_bridge::rope_bridge_solution;

mod calorie_counting;
mod rock_paper_scissors;
mod rucksack_reorganisation;
mod camp_cleanup;
mod supply_stacks;
mod tuning_trouble;
mod no_space_left;
mod treetop_tree_house;
mod rope_bridge;


fn main() {
    println!("*** Advent of Code 2022 ***");
    calorie_counting_solution();
    rock_paper_scissors_solution();
    rucksack_reorganisation_solution();
    camp_cleanup_solution();
    supply_stacks_solution();
    tuning_trouble_solution();
    no_space_left_solution();
    treetop_tree_house_solution();
    rope_bridge_solution();

}