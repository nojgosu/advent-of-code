use std::fs;
use std::ops::{IndexMut};
use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1, digit1, space0};
use nom::combinator::{map_res};
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};


pub fn solve_first_star() -> String {
    let (mut stacks, instructions) = parse_input("src/supply_stacks/input.txt");

    for instruction in instructions {
        let mut moving_containers = Vec::<char>::new();

        for _ in 0..instruction.move_count {
            moving_containers.push(stacks.index_mut(instruction.from_location - 1).pop().unwrap());
        }

        stacks.index_mut(instruction.to_location - 1).append(&mut moving_containers);
    }

    let mut result = String::new();

    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }

    result
}


pub fn solve_second_star() -> String {
    let (mut stacks, instructions) = parse_input("src/supply_stacks/input.txt");

    for instruction in instructions {
        let mut moving_containers = Vec::<char>::new();

        for _ in 0..instruction.move_count {
            moving_containers.push(stacks.index_mut(instruction.from_location - 1).pop().unwrap());
        }

        moving_containers.reverse();

        stacks.index_mut(instruction.to_location - 1).append(&mut moving_containers);
    }

    let mut result = String::new();

    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }

    result
}

fn parse_instruction(input: &str) -> Instruction {
    let mut parser = tuple((move_command, from_command, to_command));

    let (_, (move_count, from_location, to_location)) = parser(input).unwrap();

    Instruction {
        move_count,
        from_location,
        to_location,
    }
}

fn move_command(input: &str) -> IResult<&str, usize> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("move")(rest)?;
    let (rest, _) = space0(rest)?;
    let (rest, move_value) = parse_usize(rest)?;

    Ok((rest, move_value))
}

fn from_command(input: &str) -> IResult<&str, usize> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("from")(rest)?;
    let (rest, _) = space0(rest)?;
    let (rest, move_value) = parse_usize(rest)?;

    Ok((rest, move_value))
}

fn to_command(input: &str) -> IResult<&str, usize> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("to")(rest)?;
    let (rest, _) = space0(rest)?;
    let (rest, move_value) = parse_usize(rest)?;

    Ok((rest, move_value))
}


fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_char(input: &str) -> IResult<&str, char> { map_res(alpha1, str::parse)(input) }

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (rest, crate_char) = delimited(tag("["), parse_char, tag("]"))(input)?;

    Ok((rest, Some(crate_char)))
}

fn parse_gap(input: &str) -> IResult<&str, Option<char>> {
    let (rest, _) = tag("   ")(input)?;

    Ok((rest, None))
}


fn parse_supply_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let mut result = Vec::<Option<char>>::new();
    let mut remaining = input;

    let mut parser = alt((
        preceded(tag(" "), parse_gap),
        parse_crate,
        parse_gap,
        preceded(tag(" "), parse_crate),
    ));

    while !remaining.is_empty() {
        let (rest, supply) = parser(remaining)?;

        remaining = rest;

        result.push(supply);
    }

    Ok((remaining, result))
}

#[derive(Debug, PartialEq)]
struct Instruction {
    move_count: usize,
    from_location: usize,
    to_location: usize,
}


fn parse_input(file_path: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut supply_rows = Vec::<Vec<Option<char>>>::new();
    let mut instructions = Vec::<Instruction>::new();

    // Parse supply stacks
    for line in contents.lines().take(8) {
        if let Ok((rest, supply_row)) = parse_supply_row(line) {
            supply_rows.push(supply_row);
        } else {
            panic!("Error parsing supply rows")
        }
    }

    // transpose supply rows to get stacks
    supply_rows.reverse();

    let stacks = transpose(supply_rows);

    // de-option vector using flatten trick
    let stacks = stacks
        .into_iter()
        .map(|x| x.into_iter().flatten().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    // parse instructions
    for line in contents.lines().skip(10) {
        instructions.push(parse_instruction(line));
    }

    (stacks, instructions)
}


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!("MQSHJMWNH", solve_first_star());
        assert_eq!("LLWJRBHVZ", solve_second_star());
    }

    #[test]
    fn parsing_instruction() {
        assert_eq!(Instruction { move_count: 3, from_location: 5, to_location: 2 },
                   parse_instruction("move 3 from 5 to 2"));

        assert_eq!(Instruction { move_count: 5, from_location: 3, to_location: 1 },
                   parse_instruction("move 5 from 3 to 1"));

        assert_eq!(Instruction { move_count: 4, from_location: 4, to_location: 9 },
                   parse_instruction("move 4 from 4 to 9"));
    }
}