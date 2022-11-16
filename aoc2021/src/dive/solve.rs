use std::fs;

pub fn solve_first_star() -> i32 {
    let commands = parse_input("src/dive/input.txt");

    let mut distance = 0i32;
    let mut depth = 0i32;

    for command in commands {
        let command_split: Vec<&str> = command.split(' ').collect();

        match command_split[0] {
            "forward" => {
                distance += command_split[1].parse::<i32>().unwrap();
            }
            "up" => {
                depth -= command_split[1].parse::<i32>().unwrap();
            }
            "down" => {
                depth += command_split[1].parse::<i32>().unwrap();
            }
            _ => {
                panic!("Invalid submarine command");
            }
        }
    }

    return distance * depth;
}


pub fn solve_second_star() -> i32 {
    let commands = parse_input("src/dive/input.txt");

    let mut distance = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for command in commands {
        let command_split: Vec<&str> = command.split(' ').collect();

        match command_split[0] {
            "forward" => {
                distance += command_split[1].parse::<i32>().unwrap();
                depth += aim * command_split[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim -= command_split[1].parse::<i32>().unwrap();
            }
            "down" => {
                aim += command_split[1].parse::<i32>().unwrap();
            }
            _ => {
                panic!("Invalid submarine command");
            }
        }
    }

    distance * depth
}


fn parse_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let result: Vec<String> = contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(1936494, solve_first_star());
        assert_eq!(1997106066, solve_second_star());
    }
}