use std::fs;
use std::ops::{IndexMut};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map_res};
use nom::IResult;
use nom::sequence::{tuple};


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


#[derive(Debug, PartialEq)]
struct Instruction {
    move_count: usize,
    from_location: usize,
    to_location: usize,
}


fn parse_input(file_path: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut stacks = Vec::<Vec<char>>::new();
    let mut instructions = Vec::<Instruction>::new();


    // Initialise stacks
    stacks.push(vec!['B', 'W', 'N']);
    stacks.push(vec!['L', 'Z', 'S', 'P', 'T', 'D', 'M', 'B']);
    stacks.push(vec!['Q', 'H', 'Z', 'W', 'R']);
    stacks.push(vec!['W', 'D', 'V', 'J', 'Z', 'R']);
    stacks.push(vec!['S', 'H', 'M', 'B']);
    stacks.push(vec!['L', 'G', 'N', 'J', 'H', 'V', 'P', 'B']);
    stacks.push(vec!['J', 'Q', 'Z', 'F', 'H', 'D', 'L', 'S']);
    stacks.push(vec!['W', 'S', 'F', 'J', 'G', 'Q', 'B']);
    stacks.push(vec!['Z', 'W', 'M', 'S', 'C', 'D', 'J']);


    // parse instructions
    for line in contents.lines().skip(10) {
        instructions.push(parse_instruction(line));
    }

    (stacks, instructions)
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