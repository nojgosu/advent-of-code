use std::collections::HashSet;
use std::fs;
use std::ops::Index;
use std::panic::resume_unwind;
use itertools::Itertools;


pub fn solve_first_star() -> u32 {
    let data = parse_input("src/seven_segment_search/input.txt");

    // We know that there are 4 digits with unique number of lit up segments
    // 1 -> 2 segments
    // 4 -> 4 segments
    // 7 -> 3 segments
    // 8 -> 7 segments

    // Using this information, we search for how many times these digits appear in the output
    // (e.g. data after the '|')

    let mut count = 0;

    for item in data {
        let screen_readout = item
            .split('|')
            .last().unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();

        count += screen_readout.iter().filter(|x| x.len() == 2).count(); // 1 Digit
        count += screen_readout.iter().filter(|x| x.len() == 4).count(); // 4 Digit
        count += screen_readout.iter().filter(|x| x.len() == 3).count(); // 7 Digit
        count += screen_readout.iter().filter(|x| x.len() == 7).count(); // 8 Digit
    }

    count as u32
}


pub fn solve_second_star() -> u32 {
    let data = parse_input("src/seven_segment_search/input.txt");

    let mut result = 0u32;

    for reading in data {
        let decoding_key = decode_segment_wiring(&reading);

        let number = decode_reading(reading, decoding_key);

        result += number;
    }

    result
}

fn decode_reading(reading: String, decoding_key: Vec<String>) -> u32 {

    let screen_readout = reading
        .split('|')
        .last()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut number = Vec::<u32>::new();

    for reading in screen_readout.iter() {
        let digit_wiring = reading.chars().sorted().collect::<String>();


        if digit_wiring == *decoding_key.index(0) {
            number.push(0);
        } else if digit_wiring == *decoding_key.index(1) {
            number.push(1);
        } else if digit_wiring == *decoding_key.index(2) {
            number.push(2);
        } else if digit_wiring == *decoding_key.index(3) {
            number.push(3);
        } else if digit_wiring == *decoding_key.index(4) {
            number.push(4);
        } else if digit_wiring == *decoding_key.index(5) {
            number.push(5);
        } else if digit_wiring == *decoding_key.index(6) {
            number.push(6);
        } else if digit_wiring == *decoding_key.index(7) {
            number.push(7);
        } else if digit_wiring == *decoding_key.index(8) {
            number.push(8);
        } else if digit_wiring == *decoding_key.index(9) {
            number.push(9);
        }
    }

    number.iter().fold(0, |acc, elem| acc * 10 + elem)
}

//let (tr, t, tl, m, br, bl, b) = decode_segment_wiring(&item);
fn decode_segment_wiring(reading: &String) -> Vec<String> {
    // Create hashset for each of the segment and initialise with full spread of chars
    let segment_wiring = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut top_segment = HashSet::<char>::from(segment_wiring);
    let mut top_left_segment = HashSet::<char>::from(segment_wiring);
    let mut top_right_segment = HashSet::<char>::from(segment_wiring);
    let mut middle_segment = HashSet::<char>::from(segment_wiring);
    let mut bottom_left_segment = HashSet::<char>::from(segment_wiring);
    let mut bottom_right_segment = HashSet::<char>::from(segment_wiring);
    let mut bottom_segment = HashSet::<char>::from(segment_wiring);

    // Grab digit wiring list
    let mut digit_wiring = reading
        .split('|')
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();

    // sort digit_wiring so we process most informative digits first
    digit_wiring.sort_by_key(|a| a.len());

    // Decode digit wiring list
    for wiring in digit_wiring {
        match wiring.len() {
            2 => {
                // 1 Digit
                let wires = HashSet::<char>::from_iter(wiring.chars());

                // Reduce 1 segments to wires
                top_right_segment = top_right_segment.intersection(&wires).copied().collect();
                bottom_right_segment = bottom_right_segment.intersection(&wires).copied().collect();

                // Remove wires from segments it couldn't be
                top_segment = top_segment.difference(&wires).copied().collect();
                top_left_segment = top_left_segment.difference(&wires).copied().collect();
                middle_segment = middle_segment.difference(&wires).copied().collect();
                bottom_left_segment = bottom_left_segment.difference(&wires).copied().collect();
                bottom_segment = bottom_segment.difference(&wires).copied().collect();
            }
            3 => {
                // 7 Digit
                let wires = HashSet::<char>::from_iter(wiring.chars());

                // Reduce 7 segments to wires
                top_segment = top_segment.intersection(&wires).copied().collect();
                top_right_segment = top_right_segment.intersection(&wires).copied().collect();
                bottom_right_segment = bottom_right_segment.intersection(&wires).copied().collect();

                // Remove wires from segments it couldn't be
                top_left_segment = top_left_segment.difference(&wires).copied().collect();
                middle_segment = middle_segment.difference(&wires).copied().collect();
                bottom_left_segment = bottom_left_segment.difference(&wires).copied().collect();
                bottom_segment = bottom_segment.difference(&wires).copied().collect();
            }
            4 => {
                // 4 Digit
                let wires = HashSet::<char>::from_iter(wiring.chars());

                // Reduce 4 segments to wires
                top_right_segment = top_right_segment.intersection(&wires).copied().collect();
                top_left_segment = top_left_segment.intersection(&wires).copied().collect();
                middle_segment = middle_segment.intersection(&wires).copied().collect();
                bottom_right_segment = bottom_right_segment.intersection(&wires).copied().collect();

                // Remove wires from segments it couldn't be
                top_segment = top_segment.difference(&wires).copied().collect();
                bottom_left_segment = bottom_left_segment.difference(&wires).copied().collect();
                bottom_segment = bottom_segment.difference(&wires).copied().collect();
            }
            5 => {
                // 3, 5 or 2 Digit
                let wires = HashSet::<char>::from_iter(wiring.chars());

                // can uniquely identify 3 as it's the only digit to contain one
                if wires.is_superset(&top_right_segment) && wires.is_superset(&bottom_right_segment) {
                    // Found 3, reduce 3 segments to wires
                    top_segment = top_segment.intersection(&wires).copied().collect();
                    top_right_segment = top_right_segment.intersection(&wires).copied().collect();
                    middle_segment = middle_segment.intersection(&wires).copied().collect();
                    bottom_right_segment = bottom_right_segment.intersection(&wires).copied().collect();
                    bottom_segment = bottom_segment.intersection(&wires).copied().collect();

                    // Remove wires from segments it couldn't be
                    top_left_segment = top_left_segment.difference(&wires).copied().collect();
                    bottom_left_segment = bottom_left_segment.difference(&wires).copied().collect();
                }
            }
            6 => {
                // 6, 9, 0
                let wires = HashSet::<char>::from_iter(wiring.chars());

                // can uniquely identify 6 as it doesn't contain one
                if !(wires.is_superset(&top_right_segment) && wires.is_superset(&bottom_right_segment)) {
                    // Found 6, reduce 6 segments to wires
                    top_segment = top_segment.intersection(&wires).copied().collect();
                    top_left_segment = top_left_segment.intersection(&wires).copied().collect();
                    middle_segment = middle_segment.intersection(&wires).copied().collect();
                    bottom_left_segment = bottom_left_segment.intersection(&wires).copied().collect();
                    bottom_right_segment = bottom_right_segment.intersection(&wires).copied().collect();
                    bottom_segment = bottom_segment.intersection(&wires).copied().collect();

                    // Remove wires from segments it couldn't be
                    top_right_segment = top_right_segment.difference(&wires).copied().collect();
                }
            }
            7 => {
                // 8 Digit - Not informative. Do nothing.
            }
            _ => { panic!("Issue with digit wiring decoding") }
        };
    }

    // extract segment wiring solution
    let tr = top_right_segment.drain().next().unwrap();
    let t = top_segment.drain().next().unwrap();
    let tl = top_left_segment.drain().next().unwrap();
    let m = middle_segment.drain().next().unwrap();
    let br = bottom_right_segment.drain().next().unwrap();
    let bl = bottom_left_segment.drain().next().unwrap();
    let b = bottom_segment.drain().next().unwrap();

    // construct numbers using the decoded segment wiring, sort and cast as a String
    let zero = vec![t, tl, tr, br, bl, b].into_iter().sorted().collect::<String>();
    let one = vec![tr, br].into_iter().sorted().collect::<String>();
    let two = vec![t, tr, m, bl, b].into_iter().sorted().collect::<String>();
    let three = vec![t, tr, m, br, b].into_iter().sorted().collect::<String>();
    let four = vec![tl, tr, m, br].into_iter().sorted().collect::<String>();
    let five = vec![t, tl, m, br, b].into_iter().sorted().collect::<String>();
    let six = vec![t, tl, m, br, bl, b].into_iter().sorted().collect::<String>();
    let seven = vec![t, tr, br].into_iter().sorted().collect::<String>();
    let eight = vec![t, tl, tr, m, br, bl, b].into_iter().sorted().collect::<String>();
    let nine = vec![t, tl, tr, m, br, b].into_iter().sorted().collect::<String>();

    let decoding_key = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    decoding_key
}


fn parse_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    contents
        .lines()
        .map(String::from)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(440, solve_first_star());
        assert_eq!(1046281, solve_second_star());
    }
}