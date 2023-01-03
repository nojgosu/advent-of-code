use std::fs;
use ndarray::{Array2, s};
use nom::bytes::complete::{tag};
use nom::character::complete;
use nom::character::complete::digit1;
use nom::combinator::{rest};
use nom::error::Error;

use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;


pub fn solve_first_star() -> usize {
    let (mut cave_system, highest_y) = parse_input("src/regolith_reservoir/input.txt");

    while drop_sand(&mut cave_system) {}

    // count sand
    cave_system.iter().filter(|&x| *x == 'o' ).count()

}


pub fn solve_second_star() -> usize {
    let (mut cave_system, highest_y)  = parse_input("src/regolith_reservoir/input.txt");

    let (_, max_col) = cave_system.dim();

    // add cave floor
    draw_rock(&mut cave_system, vec![(0, highest_y+2),(max_col-1, highest_y+2)]);

    while drop_sand(&mut cave_system) {}

    // count sand
    cave_system.iter().filter(|&x| *x == 'o' ).count()

}


fn drop_sand(cave: &mut Array2<char>) -> bool {

    // simulate sand dropping
    let origin = (500_usize, 0_usize);

    // check if cave is full
    if cave[[origin.1, origin.0]] == 'o' {
        return false;
    }

    let mut sand_pos = origin;
    loop {
        let new_sand_pos = simulate_sand_step(cave, sand_pos);
        if let Some(sand) = new_sand_pos {
            if sand == sand_pos {
                //settled, update cave and return true;
                cave[[sand.1, sand.0]] = 'o';
                //println!("setted at {:?}", sand);
                return true;
            }
        } else {
            // sand hit the abyss, return false
            //println!("abyssed at {:?}", origin);
            return false;
        }
        sand_pos = new_sand_pos.unwrap();
    }
}


fn simulate_sand_step(cave: &Array2<char>, current: (usize, usize)) -> Option<(usize, usize)> {
    // check for out of bounds
    let (row, col) = cave.dim();

    if current.1 + 1 >= row {
        return None;
    }

    let down = (current.0 as i64, current.1 as i64 + 1);
    let down_left = (current.0 as i64 - 1, current.1 as i64 + 1);
    let down_right = (current.0 as i64 + 1, current.1 as i64 + 1);

    // check move list
    if cave[[down.1 as usize, down.0 as usize]] == '.' {
        Some((down.0 as usize, down.1 as usize))
    } else if cave[[down_left.1 as usize, down_left.0 as usize]] == '.' {
        Some((down_left.0 as usize, down_left.1 as usize))
    } else if cave[[down_right.1 as usize, down_right.0 as usize]] == '.' {
        Some((down_right.0 as usize, down_right.1 as usize))
    } else {
        Some(current)
    }
}


fn parse_input(file_path: &str) -> (Array2<char>, usize) {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let size = (200, 900);

    let mut result = Array2::<char>::from_elem(size, '.');

    let mut highest_y = 0_usize;

    for line in content.lines() {
        let parsed_cave_row = parse_cave_row(line);

        match parsed_cave_row {
            Ok((_, rock_scan)) => {
                let new_highest = rock_scan.iter()
                    .max_by(|&a,&b| a.1.cmp(&b.1));

                if new_highest.unwrap().1 >= highest_y {highest_y = new_highest.unwrap().1}

                draw_rock(&mut result, rock_scan);
            }
            _ => { panic!("Error: failed to parse cave row") }
        }
    }

    (result, highest_y)
}

fn draw_rock(cave: &mut Array2<char>, rock_scan: Vec<(usize, usize)>) {
    for window in rock_scan.as_slice().windows(2) {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];

        let dx = x2 as i64 - x1 as i64;
        let dy = y2 as i64 - y1 as i64;

        let x_start;
        let y_start;

        if dx >= 0 {
            x_start = x1;
        } else {
            x_start = x2;
        }

        if dy >= 0 {
            y_start = y1;
        } else {
            y_start = y2;
        }

        for i in 0..dy.abs() as usize + 1 {
            cave[[y_start + i, x_start]] = '#';
        }

        for i in 0..dx.abs() as usize + 1 {
            cave[[y_start, x_start + i]] = '#';
        }
    }
}

fn parse_cave_row(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    // 489,96 -> 489,98 -> 482,98
    let parser = separated_pair(parse_usize, tag(","), parse_usize);

    let (remaining, rock_path) = separated_list1(tag(" -> "), parser)(input)?;

    Ok((remaining, rock_path))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (rest, num) = digit1(input)?;

    let (_, num) = complete::u64::<_, Error<_>>(num).unwrap();

    Ok((rest, num as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(618, solve_first_star());
        assert_eq!(26358, solve_second_star());
    }
}