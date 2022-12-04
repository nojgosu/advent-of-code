use std::collections::{HashSet};
use std::fs;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, digit1};
use nom::error::Error;
use nom::combinator::{map, map_res, rest};
use nom::IResult;
use nom::sequence::separated_pair;

pub fn solve_first_star() -> u64 {
    let assignment_pairs = parse_input("src/camp_cleanup/input.txt");

    let mut redundant_assignment = 0;

    for (assignment_1, assignment_2) in assignment_pairs {
        let overlap = assignment_1.intersection(&assignment_2).count();

        if overlap == assignment_1.len() || overlap == assignment_2.len() {
            redundant_assignment += 1;
        }
    }

    redundant_assignment
}


pub fn solve_second_star() -> u64 {
    let assignment_pairs = parse_input("src/camp_cleanup/input.txt");

    let mut inefficient_assignment = 0;

    for (assignment_1, assignment_2) in assignment_pairs {
        let overlap = assignment_1.intersection(&assignment_2).count();

        if overlap != 0 {
            inefficient_assignment += 1;
        }
    }

    inefficient_assignment
}


fn parse_assignment(input: &str) -> IResult<&str, HashSet<u32>> {
    let range_parser = separated_pair(parse_number, char('-'), parse_number);

    let mut assignment_parser = map(range_parser, |(start, end)| {
        HashSet::<u32>::from_iter((start..end + 1).collect::<Vec<_>>())
    });

    assignment_parser(input)
}


fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}


fn get_assignments(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(is_not(","), tag(","), rest)(input)
}


fn parse_input(file_path: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<(HashSet<u32>, HashSet<u32>)>::new();

    for line in contents.lines() {
        if let Ok((_, (first, second))) = get_assignments(line) {
            let (_, assignment_1) = parse_assignment(first).unwrap();
            let (_, assignment_2) = parse_assignment(second).unwrap();

            result.push((assignment_1, assignment_2));
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use nom::combinator::rest;
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(494, solve_first_star());
        assert_eq!(833, solve_second_star());
    }

    #[test]
    fn parsing_assignment() {
        let mut parser = separated_pair::<_, _, _, _, Error<_>, _, _, _>(is_not(","), tag(","), rest);

        assert_eq!(parser("36-92,35-78"), Ok(("", ("36-92", "35-78"))));
    }
}