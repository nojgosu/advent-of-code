use std::collections::{HashSet, VecDeque};
use std::fs;


pub fn solve_first_star() -> u32 {
    let rucksacks = parse_input("src/rucksack_reorganisation/input.txt");

    let mut priorities_value = 0_u32;

    for rucksack in rucksacks {
        let (left_compartment, right_compartment) = get_compartments(rucksack);
        let mut duplicate_item: Option<char> = None;

        for c in right_compartment {
            if left_compartment.contains(&c) {
                duplicate_item = Some(c);
            }
        }

        priorities_value += calculate_priority(duplicate_item.unwrap());
    }

    priorities_value
}


pub fn solve_second_star() -> u32 {
    let mut rucksacks = parse_input("src/rucksack_reorganisation/input.txt");

    let mut badge_priorities_value = 0_u32;


    // get 3 groups
    while !rucksacks.is_empty() {
        let group1 = HashSet::<char>::from_iter(rucksacks.pop_front().unwrap().into_iter());
        let group2 = HashSet::<char>::from_iter(rucksacks.pop_front().unwrap().into_iter());
        let group3 = HashSet::<char>::from_iter(rucksacks.pop_front().unwrap().into_iter());

        let badge_candidates = group1.intersection(&group2).copied().collect::<HashSet<_>>();

        if let Some(badge) = badge_candidates.intersection(&group3).next() {
            badge_priorities_value += calculate_priority(*badge);
        }
    }

    badge_priorities_value
}


fn calculate_priority(duplicate_item: char) -> u32 {
    // cast to u32 and offset ascii value to get correct priority
    let priority_ascii_offset = if duplicate_item.is_lowercase() {
        96
    } else {
        38
    };

    duplicate_item as u32 - priority_ascii_offset
}


fn get_compartments(rucksack: Vec<char>) -> (Vec<char>, Vec<char>) {
    let rucksack_size = rucksack.len();

    let left_compartment = rucksack.iter().copied().take(rucksack_size / 2).collect::<Vec<_>>();
    let right_compartment = rucksack.iter().copied().skip(rucksack_size / 2).collect::<Vec<_>>();

    (left_compartment, right_compartment)
}


fn parse_input(file_path: &str) -> VecDeque<Vec<char>> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut rucksacks = VecDeque::<Vec<char>>::new();

    for line in contents.lines() {
        rucksacks.push_back(line.chars().collect::<Vec<_>>())
    }

    rucksacks
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(7850, solve_first_star());
        assert_eq!(2581, solve_second_star());
    }
}