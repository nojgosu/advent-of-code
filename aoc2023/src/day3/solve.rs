use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use std::collections::HashSet;
use std::fs;

pub fn solve_first_star() -> usize {
    let (symbols, numbers) = parse_input("src/day3/input.txt");

    let part_numbers = identify_part_numbers(symbols, numbers);

    part_numbers.iter().sum()
}

fn identify_part_numbers(symbols: Vec<Symbol>, numbers: Vec<Number>) -> Vec<usize> {
    // extract symbol locations
    let symbol_locations: HashSet<Point> = symbols.iter().map(|s| s.location.clone()).collect();

    let mut part_numbers = Vec::<usize>::new();

    numbers.iter().for_each(|num| {
        if num
            .proximate_points()
            .iter()
            .any(|x| symbol_locations.contains(x))
        {
            part_numbers.push(num.value);
        }
    });
    part_numbers
}

pub fn solve_second_star() -> usize {
    let (symbols, numbers) = parse_input("src/day3/input.txt");

    let gear_ratios = calculate_gear_ratios(symbols, numbers);

    gear_ratios.iter().sum()
}

fn calculate_gear_ratios(symbols: Vec<Symbol>, numbers: Vec<Number>) -> Vec<usize> {
    // extract gear locations
    let possible_gears: Vec<&Symbol> = symbols.iter().filter(|&x| x.value == '*').collect();

    let mut gear_ratios = Vec::<usize>::new();

    // check each symbol if its a gear
    possible_gears.iter().for_each(|&gear| {
        let parts: Vec<&Number> = numbers
            .iter()
            .filter(|&num| num.proximate_points().contains(&gear.location))
            .collect();

        if parts.len() == 2 {
            gear_ratios.push(parts.iter().map(|&x| x.value).product());
        }
    });
    gear_ratios
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct Span {
    start: Point,
    end: Point,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    location: Point,
    value: char,
}

#[derive(PartialEq, Debug)]
struct Number {
    location: Span,
    value: usize,
}

impl Number {
    fn proximate_points(&self) -> Vec<Point> {
        let mut result = Vec::<Point>::new();

        // set up x,y coordinate extents
        let mut x1 = self.location.start.x;
        let x2 = self.location.end.x + 1;
        let mut y1 = self.location.start.y;
        let y2 = self.location.end.y + 1;

        if x1 > 0 {
            x1 -= 1
        }
        if y1 > 0 {
            y1 -= 1
        }

        // enumerate extents generating points
        for x in x1..=x2 {
            for y in y1..=y2 {
                result.push(Point { x, y })
            }
        }

        result
    }
}

fn parse_input(file_path: &str) -> (Vec<Symbol>, Vec<Number>) {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut symbols = Vec::<Symbol>::new();
    let mut numbers = Vec::<Number>::new();

    // return symbols and their location
    // return numbers and their span

    let mut x;
    let mut y = 0_usize;

    for line in content.lines() {
        // I need track location
        // reset x coordinate
        x = 0;

        // many0 with parsers to completely consume line
        let (_left, engine_schematic_line) = parse_engine_schematic_line(line).unwrap();

        // interpret parsed results
        engine_schematic_line.iter().for_each(|&token| match token {
            "%" | "*" | "/" | "+" | "=" | "#" | "$" | "&" | "@" | "-" => {
                symbols.push(Symbol {
                    location: Point { x, y },
                    value: token.chars().next().unwrap(),
                });

                x += 1;
            }
            "." => {
                // discard and increment x location by 1
                x += 1;
            }
            num => {
                numbers.push(Number {
                    value: num.parse::<usize>().unwrap(),
                    location: Span {
                        start: Point { x, y },
                        end: Point {
                            x: x + num.len() - 1,
                            y,
                        },
                    },
                });

                // increment x location by the length of the number
                x += num.len();
            }
        });

        // increment y coordinate
        y += 1;
    }

    (symbols, numbers)
}

fn parse_engine_schematic_line(input: &str) -> IResult<&str, Vec<&str>> {
    //many0(alt((tag("."), digit1, one_of("%"))))(input)
    many0(alt((tag("."), digit1, parse_symbol)))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, &str> {
    alt((
        tag("%"),
        tag("*"),
        tag("/"),
        tag("+"),
        tag("="),
        tag("#"),
        tag("$"),
        tag("&"),
        tag("@"),
        tag("-"),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(559667, solve_first_star());
        assert_eq!(86841457, solve_second_star());
    }

    #[test]
    fn test_solution_1() {
        let (symbols, numbers) = parse_input("src/day3/test_input.txt");

        let part_numbers = identify_part_numbers(symbols, numbers);

        assert_eq!(4361_usize, part_numbers.iter().sum());
    }

    #[test]
    fn test_solution_2() {
        let (symbols, numbers) = parse_input("src/day3/test_input.txt");

        let gear_ratios = calculate_gear_ratios(symbols, numbers);

        assert_eq!(467835_usize, gear_ratios.iter().sum());
    }
}
