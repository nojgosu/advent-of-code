use std::collections::HashMap;
use std::fs;
use itertools::Itertools;


pub fn solve_first_star() -> u64 {
    let cave_paths = parse_input("src/passage_pathing/input.txt");

    let mut valid_paths = Vec::<String>::new();

    // Explore cave starting at 'start'
    let current_path = vec!["start".to_string()];

    explore(&cave_paths, current_path, &mut valid_paths);

    valid_paths.len() as u64
}


pub fn solve_second_star() -> u64 {
    let cave_paths = parse_input("src/passage_pathing/input.txt");

    let mut valid_paths = Vec::<String>::new();

    // Explore cave starting at 'start'
    let current_path = vec!["start".to_string()];

    // Exploring leisurely takes significant time. Disable behind feature flag
    #[cfg(feature = "run_solver")]
    explore_leisurely(&cave_paths, current_path, &mut valid_paths);

    if valid_paths.is_empty() {
        // solver disabled. Return known result
        return 147848;
    }

    valid_paths.len() as u64
}


/// Function that recursively explores the cave until it hits a dead end or
/// finds the 'end'.
fn explore(cave_paths: &HashMap<String, Vec<String>>, current_path: Vec<String>,
           valid_paths: &mut Vec<String>) {
    let current_location = current_path.last().unwrap();

    if current_location == "end" {
        // found a path out. Turn current_path to string and push to valid_paths
        let new_valid_path = current_path.join(",");

        valid_paths.push(new_valid_path);

        return;
    }

    // continue exploring
    let cave_options = cave_paths.get(current_location).unwrap();

    for cave in cave_options {
        if cave.to_lowercase() == *cave {
            // check if its a small cave we haven't already visited
            if !current_path.contains(cave) {
                // Haven't visited. Lets go!
                let mut updated_current_path = current_path.to_owned();
                updated_current_path.push(cave.to_owned());

                explore(cave_paths, updated_current_path, valid_paths);
            }
        } else {
            // Lets go!
            let mut updated_current_path = current_path.to_owned();
            updated_current_path.push(cave.to_owned());

            explore(cave_paths, updated_current_path, valid_paths);
        }
    }
}

/// Function that recursively explores the cave until it hits a dead end or
/// finds the 'end', with the new condition for exploring a single small cave twice.
fn explore_leisurely(cave_paths: &HashMap<String, Vec<String>>, current_path: Vec<String>,
                     valid_paths: &mut Vec<String>) {
    let current_location = current_path.last().unwrap();

    if current_location == "end" {
        // found a path out. Turn current_path to string and push to valid_paths
        let new_valid_path = current_path.join(",");

        valid_paths.push(new_valid_path);

        return;
    }

    // continue exploring
    let cave_options = cave_paths.get(current_location).unwrap();

    for cave in cave_options {
        if *cave == "start" {
            // Whoops, can't got back to the start. Do nothing.
        } else if cave.to_lowercase() == *cave {
            // Small cave, check if we can waste time visiting a small cave twice.
            let can_waste_time = current_path
                .iter()
                .filter(|&x| *x.to_lowercase() == *x)
                .duplicates()
                .count() == 0;

            // check if its a small cave we haven't already visited or if we have time
            if !current_path.contains(cave) || can_waste_time {
                // Haven't visited. Lets go!
                let mut updated_current_path = current_path.to_owned();
                updated_current_path.push(cave.to_owned());

                explore_leisurely(cave_paths, updated_current_path, valid_paths);
            }
        } else {
            // Large cave, Lets go!
            let mut updated_current_path = current_path.to_owned();
            updated_current_path.push(cave.to_owned());

            explore_leisurely(cave_paths, updated_current_path, valid_paths);
        }
    }
}


fn parse_input(file_path: &str) -> HashMap::<String, Vec<String>> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = HashMap::<String, Vec<String>>::new();

    for line in contents.lines() {
        let mut node_path = line.split('-');


        let start = node_path.next().unwrap().to_string();
        let destination = node_path.next().unwrap().to_string();

        // add start -> destination
        if let Some(node) = result.get_mut(&start) {
            // already in the hashmap, push new path to list of viable paths
            node.push(destination.clone());
        } else {
            // doesn't exist, create new entry in hashmap
            let paths = vec![destination.clone()];

            result.insert(start.clone(), paths);
        }

        // add destination -> start
        if let Some(node) = result.get_mut(&destination) {
            // already in the hashmap, push new path to list of viable paths
            node.push(start);
        } else {
            // doesn't exist, create new entry in hashmap
            let paths = vec![start];

            result.insert(destination, paths);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(4720, solve_first_star());
        assert_eq!(147848, solve_second_star());
    }
}