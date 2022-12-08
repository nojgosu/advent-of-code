use std::collections::{HashMap};
use std::fs;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha0, alpha1, digit1, not_line_ending, space0, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::separated_pair;
use crate::no_space_left::solve::CommandLineValue::{ChangeDirectoryCommand, Directory, File, ListCommand};


pub fn solve_first_star() -> usize {
    let commands = parse_input("src/no_space_left/input.txt");

    let mut paths = calculate_disk_usage(commands);

    paths.retain(|_, size| *size < 100000);

    let result = paths.iter().fold(0, |acc, (_, size)| acc + *size);

    result
}


pub fn solve_second_star() -> usize {
    let commands = parse_input("src/no_space_left/input.txt");

    let mut paths = calculate_disk_usage(commands);

    // total disk usage
    let disk_size = 70000000_usize;
    let disk_usage = paths.get("/").unwrap();
    let remaining_space = disk_size - disk_usage;
    let required_space = 30000000_usize;
    let delete_space = required_space - remaining_space;

    paths.retain(|_, size| *size >= delete_space);

    let (_, ans) = paths.iter().sorted_by(|(_, a), (_, b)| a.cmp(b)).next().unwrap();

    *ans
}


fn calculate_disk_usage(commands: String) -> HashMap<String, usize> {
    let mut pwd = Vec::<&str>::new();
    let mut paths = HashMap::<String, usize>::new();


    for command_line in commands.lines() {
        let command_line_value = parse_command_line(command_line);

        match command_line_value {
            ChangeDirectoryCommand(dir) => {
                // add directory to present working directory
                if dir == ".." {
                    pwd.pop();
                } else {
                    pwd.push(dir);
                }
            }
            Directory(name) => {}
            File(_, _, size) => {
                // add size to directory listing in pwd
                let all_paths = enumerate_paths(&pwd);

                all_paths.iter().for_each(|dir| {
                    *paths.entry(dir.to_string()).or_insert(0) += size;
                    //*dir_size += size;
                });
            }
            ListCommand => {}
            _ => {}
        }
    }
    paths
}


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum CommandLineValue<'a> {
    File(&'a str, &'a str, usize),
    Directory(&'a str),
    ChangeDirectoryCommand(&'a str),
    ListCommand,
}


fn enumerate_paths(path: &Vec<&str>) -> Vec<String> {
    let mut result = Vec::<String>::new();

    let mut path_iter = path.iter();

    let mut path = String::new();

    if let Some(&root) = path_iter.next() {
        // push root and add to path
        result.push(root.to_string());

        path.push_str(root);
    }

    for &dir in path_iter {
        path.push_str(dir);
        path.push('/');

        result.push(path.clone());
    }

    result
}


fn parse_command_line(input: &str) -> CommandLineValue {
    let mut parser = alt((
        cd_command,
        ls_command,
        process_ls_file,
        process_ls_file_ext,
        process_ls_dir
    ));

    let (_, result) = parser(input).unwrap();

    result
}


fn cd_command(input: &str) -> IResult<&str, CommandLineValue> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("$")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, _) = tag("cd")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, directory) = not_line_ending(rest)?;

    Ok((rest, ChangeDirectoryCommand(directory)))
}


fn ls_command(input: &str) -> IResult<&str, CommandLineValue> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("$")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, _) = tag("ls")(rest)?;
    let (rest, _) = space0(rest)?;

    Ok((rest, ListCommand))
}


fn process_ls_file_ext(input: &str) -> IResult<&str, CommandLineValue> {
    let (rest, _) = space0(input)?;
    let (rest, file_size) = parse_usize(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, (name, extension)) = separated_pair(alpha1, tag("."), alpha0)(rest)?;
    let (rest, _) = space0(rest)?;

    Ok((rest, File(name, extension, file_size)))
}


fn process_ls_file(input: &str) -> IResult<&str, CommandLineValue> {
    let (rest, _) = space0(input)?;
    let (rest, file_size) = parse_usize(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, name) = alpha0(rest)?;
    let (rest, _) = space0(rest)?;

    Ok((rest, File(name, "", file_size)))
}


fn process_ls_dir(input: &str) -> IResult<&str, CommandLineValue> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("dir")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, dir) = alpha0(rest)?;

    Ok((rest, Directory(dir)))
}


fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}


fn parse_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Input file local to project")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(1844187, solve_first_star());
        assert_eq!(4978279, solve_second_star());
    }
}