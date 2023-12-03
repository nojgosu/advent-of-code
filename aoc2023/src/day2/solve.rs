use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{digit1, space0};
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::IResult;
use std::cmp::max;
use std::fs;

use crate::day2::solve::Cube::{Blue, Green, Red};
use nom::multi::{many0, separated_list0};

pub fn solve_first_star() -> usize {
    let games = parse_input("src/day2/input.txt");

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut possible_games = Vec::<usize>::new();

    games.iter().for_each(|game| {
        let impossible_game = game
            .draws
            .iter()
            .map(|draw| {
                draw.cubes.iter().any(|cube| match cube {
                    Red(val) => *val > red_limit,
                    Blue(val) => *val > blue_limit,
                    Green(val) => *val > green_limit,
                })
            })
            .any(|impossible| impossible == true);

        if !impossible_game {
            possible_games.push(game.id)
        };
    });

    possible_games.iter().sum()
}

pub fn solve_second_star() -> usize {
    let games = parse_input("src/day2/input.txt");

    let mut game_powers = Vec::<usize>::new();

    games.iter().for_each(|game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        game.draws.iter().for_each(|draw| {
            draw.cubes.iter().for_each(|cube| match cube {
                Red(val) => {
                    max_red = max(max_red, *val);
                }
                Blue(val) => {
                    max_blue = max(max_blue, *val);
                }
                Green(val) => {
                    max_green = max(max_green, *val);
                }
            })
        });

        game_powers.push(max_red * max_blue * max_green);
    });

    game_powers.iter().sum()
}

fn parse_input(file_path: &str) -> Vec<Game> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<Game>::new();

    for line in content.lines() {
        // parse games
        if let Ok((_, game)) = parse_game_result(line) {
            result.push(game);
        }
    }

    result
}

#[derive(PartialEq, Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(PartialEq, Debug)]
struct Draw {
    cubes: Vec<Cube>,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Cube {
    Red(usize),
    Blue(usize),
    Green(usize),
}

fn parse_game_result(input: &str) -> IResult<&str, Game> {
    let (left, _) = space0(input)?;
    let (left, id) = preceded(tag("Game "), map_res(digit1, |s: &str| s.parse::<usize>()))(left)?;
    let (left, _) = space0(left)?;
    let (left, _) = tag(":")(left)?;
    let (left, _) = space0(left)?;
    let (left, draws) = parse_draws(left)?;

    Ok((left, Game { id, draws }))
}

fn parse_red_cubes(input: &str) -> IResult<&str, Cube> {
    let (left, _) = space0(input)?;
    let (left, num) = digit1(left)?;
    let (left, _) = space0(left)?;
    let (left, _) = tag("red")(left)?;
    let (left, _) = many0(tag(","))(left)?;
    Ok((left, Red(num.parse::<usize>().unwrap())))
}

fn parse_blue_cubes(input: &str) -> IResult<&str, Cube> {
    let (left, _) = space0(input)?;
    let (left, num) = digit1(left)?;
    let (left, _) = space0(left)?;
    let (left, _) = tag("blue")(left)?;
    let (left, _) = many0(tag(","))(left)?;
    Ok((left, Blue(num.parse::<usize>().unwrap())))
}

fn parse_green_cubes(input: &str) -> IResult<&str, Cube> {
    let (left, _) = space0(input)?;
    let (left, num) = digit1(left)?;
    let (left, _) = space0(left)?;
    let (left, _) = tag("green")(left)?;
    let (left, _) = many0(tag(","))(left)?;
    Ok((left, Green(num.parse::<usize>().unwrap())))
}

fn parse_draws(input: &str) -> IResult<&str, Vec<Draw>> {
    let mut result = Vec::<Draw>::new();

    // Break down input into draws
    let (left, _) = space0(input)?;
    let (left, draws) = separated_list0(tag(";"), is_not(";"))(left)?;

    // parse draws
    draws.into_iter().for_each(|draw| {
        let (_, cubes) =
            many0(alt((parse_red_cubes, parse_blue_cubes, parse_green_cubes)))(draw).unwrap();

        result.push(Draw { cubes });
    });

    Ok((left, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(2265, solve_first_star());
        assert_eq!(64097, solve_second_star());
    }

    #[test]
    fn test_input_parser() {
        if let Ok((_, result)) =
            parse_game_result("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        {
            assert_eq!(
                result,
                Game {
                    id: 1,
                    draws: Vec::from([
                        Draw {
                            cubes: Vec::from([Blue(3), Red(4)])
                        },
                        Draw {
                            cubes: Vec::from([Red(1), Green(2), Blue(6)])
                        },
                        Draw {
                            cubes: Vec::from([Green(2)])
                        }
                    ]),
                }
            );
        };
    }
}
