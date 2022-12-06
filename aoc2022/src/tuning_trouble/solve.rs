use std::collections::HashSet;
use std::fs;


pub fn solve_first_star() -> usize {
    let datastream = parse_input("src/tuning_trouble/input.txt");

    find_start_marker(datastream, 4)
}


pub fn solve_second_star() -> usize {
    let datastream = parse_input("src/tuning_trouble/input.txt");

    find_start_marker(datastream, 14)
}


fn find_start_marker(datastream: String, marker_size: usize) -> usize {
    let datastream_chars = datastream.chars().collect::<Vec<_>>();

    let window = datastream_chars.windows(marker_size);

    for marker in window {
        let unique_chars = HashSet::<&char>::from_iter(marker.iter());

        if unique_chars.len() == marker_size {
            // Found the packet marker, calculate start offset based on marker location
            let marker_pattern: String = marker.iter().collect();
            return datastream.find(&marker_pattern).unwrap() + marker_size;
        }
    }

    panic!("Error: No marker detected for size = {}", marker_size);
}


fn parse_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Input file local to project")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(1623, solve_first_star());
        assert_eq!(3774, solve_second_star());
    }
}