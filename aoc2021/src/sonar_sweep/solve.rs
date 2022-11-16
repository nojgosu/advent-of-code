use std::fs;


pub fn solve_first_star() -> u32 {
    let depths = parse_input("src/sonar_sweep/input.txt");

    let mut result = 0u32;

    // initialise previous depth as there is no previous measurement to compare against
    let mut prev_depth = depths[0];

    for depth in depths {
        if depth > prev_depth { result += 1; }

        prev_depth = depth;
    }

    result
}


pub fn solve_second_star() -> u32 {
    let depths = parse_input("src/sonar_sweep/input.txt");

    let mut result = 0u32;

    // set up sliding window of 3 measurements
    let mut depths_window = depths.windows(3);

    // initialise previous avg depth as there is no previous measurement to compare against
    let mut prev_avg_depth: u64 = depths_window.next().unwrap().iter().sum();

    for new_readings in depths_window {
        let new_avg_depth = new_readings.iter().sum();

        if new_avg_depth > prev_avg_depth { result += 1; }

        prev_avg_depth = new_avg_depth;
    }

    result
}


fn parse_input(file_path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let result = contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(1195, solve_first_star());
        assert_eq!(1235, solve_second_star());
    }
}