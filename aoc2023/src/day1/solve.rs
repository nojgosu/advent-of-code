use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{anychar, digit1};
use nom::error::Error;
use nom::{multi::many_till, IResult};
use std::{fs};

pub fn solve_first_star() -> i64 {
    let input = parse_first_star_input("src/day1/input.txt");

    input.iter().sum::<i64>()
}

pub fn solve_second_star() -> i64 {
    let input = parse_second_star_input("src/day1/input.txt");

    input.iter().sum::<i64>()
}

fn parse_first_star_input(file_path: &str) -> Vec<i64> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result: Vec<i64> = Vec::<i64>::new();

    for line in content.lines() {
        // extract first number
        let (_rest, num1) = first_star_digit_parser(line).unwrap();

        // extract second number
        let line_rev = line.chars().rev().collect::<String>();
        let (_rest, num2) = first_star_digit_parser(&line_rev).unwrap();

        let combined_number = num1.to_owned() + num2;

        // push to vector
        result.push(combined_number.parse::<i64>().unwrap());
    }

    result
}

fn parse_second_star_input(file_path: &str) -> Vec<i64> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result: Vec<i64> = Vec::<i64>::new();

    for line in content.lines() {
        let mut number = Vec::<char>::new();

        //extract first digit
        if let Ok((_, digit)) = parse_digit(line) {
            number.push(digit);
        }

        //extract last digit
        let rev_line = line.chars().rev().collect::<String>();
        if let Ok((_, digit)) = parse_digit_reverse(&rev_line) {
            number.push(digit);
        }

        // convert to number and push
        result.push(
            number
                .into_iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap(),
        );
    }

    result
}

fn first_star_digit_parser(s: &str) -> IResult<&str, &str, Error<&str>> {
    let (_left, (_, num)) = many_till(anychar, digit1)(s)?;
    take(1_usize)(num)
}

fn match_digit(s: &str) -> IResult<&str, &str> {
    alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
        digit1,
    ))(s)
}

fn parse_digit(input: &str) -> IResult<&str, char> {
    let (left, (_, digit)) = many_till(anychar, match_digit)(input)?;

    let result = convert_digit_to_char(digit);

    Ok((left, result))
}

fn match_digit_reverse(s: &str) -> IResult<&str, &str> {
    alt((
        tag("eno"),
        tag("owt"),
        tag("eerht"),
        tag("ruof"),
        tag("evif"),
        tag("xis"),
        tag("neves"),
        tag("thgie"),
        tag("enin"),
        digit1,
    ))(s)
}

fn parse_digit_reverse(input: &str) -> IResult<&str, char> {
    let (left, (_, digit)) = many_till(anychar, match_digit_reverse)(input)?;

    let result = convert_digit_to_char(digit);

    Ok((left, result))
}

fn convert_digit_to_char(digit: &str) -> char {
    match digit {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        "eno" => '1',
        "owt" => '2',
        "eerht" => '3',
        "ruof" => '4',
        "evif" => '5',
        "xis" => '6',
        "neves" => '7',
        "thgie" => '8',
        "enin" => '9',
        digit => digit.chars().next().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(55029, solve_first_star());
        assert_eq!(55686, solve_second_star()); // too low
    }

    #[test]
    fn first_star_test_parse() {
        let input = parse_first_star_input("src/day1/test_input.txt");

        assert_eq!(input, [12, 38, 15, 77]);
    }

    #[test]
    fn first_star_test_solution() {
        let input = parse_first_star_input("src/day1/test_input.txt");

        assert_eq!(input.iter().sum::<i64>(), 142);
    }

    #[test]
    fn second_star_test_parse() {
        let input = parse_second_star_input("src/day1/test_input2.txt");

        assert_eq!(input, [29, 83, 13, 24, 42, 14, 76]);
    }

    #[test]
    fn second_star_test_solution() {
        let input = parse_second_star_input("src/day1/test_input2.txt");

        assert_eq!(input.iter().sum::<i64>(), 281);
    }
}
