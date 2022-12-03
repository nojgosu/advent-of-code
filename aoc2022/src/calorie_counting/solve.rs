use std::fs;


pub fn solve_first_star() -> u64 {
    let calories = parse_input("src/calorie_counting/input.txt");

    calories.into_iter().max().unwrap()
}


pub fn solve_second_star() -> u64 {
    let mut calories = parse_input("src/calorie_counting/input.txt");

    calories.sort();

    calories.iter().rev().take(3).sum()
}

/// parse input, accumulating the calories on each elf and returning a vector entry per elf
/// with their carried calories
fn parse_input(file_path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let lines = contents
        .lines();

    let mut result = Vec::<u64>::new();

    let mut acc = 0_u64;

    for line in lines {
        if line.is_empty() {

            // ran out of food
            result.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u64>().unwrap();
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(71471, solve_first_star());
        assert_eq!(211189, solve_second_star());
    }
}