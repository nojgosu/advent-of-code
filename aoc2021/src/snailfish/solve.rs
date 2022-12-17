use core::panicking::panic;
use std::fs;
use std::ops::Add;
use nom::InputIter;
use crate::snailfish::solve::SnailfishNumber::*;


pub fn solve_first_star() -> u64 {
    let homework = parse_input("src/snailfish/input.txt");

    dbg!(&homework);

    for x in homework {
        let x = x + Number(0);

        dbg!(x);
    }

    0
}


pub fn solve_second_star() -> u64 {
    let homework = parse_input("src/snailfish/input.txt");


    0
}


// Think of snailfishnumber as a tree
// leaf or branch
// explode, => left frag goes up and left
// split => turn node into two branches

#[derive(Debug, PartialEq)]
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Explode(u32, u32),
    LeftFragment(u32),
    RightFragment(u32),
    Number(u32),
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = Pair(Box::from(self), Box::from(rhs));
        result.reduce(0)
    }
}

impl SnailfishNumber {
    fn unwrap(self) -> u32 {
        if let Number(x) = self {
            x
        } else if let LeftFragment(x) = self {
            x
        } else if let RightFragment(x) = self {
            x
        } else {
            panic!("Failure on unwrap for SnailfishNumber: {:?}", self);
        }
    }

    fn add_fragment(&mut self, fragment: SnailfishNumber) {
        if let Number(num) = self {
            *num += fragment.unwrap();
        } else if let Pair(a, b) = self {
            match fragment {
                LeftFragment(num) => {b.add_fragment(fragment)}
                RightFragment(num) => {a.add_fragment(fragment)}
                _ => { panic!("Error: wtf did you give me | fragment = {:?}", fragment); }
            };
        } else {
            panic!("Why try and add a fragment to this ? {:?}", self);
        }


    }

    fn reduce(self, depth: usize) -> SnailfishNumber {

        // need to traverse down the structure.

        // so recursively call reduce on each item in Pair, until we hit Number.
        // if Number, split or return Number
        // if explode, handle explode.

        match self {
            Pair(a, b) => {
                if depth >= 4 {
                    // Explode Snailfish Number
                    // Assumption: Exploding pairs will always consist of two regular numbers.
                    let x = a.unwrap();
                    let y = b.unwrap();

                    Explode(x, y)
                } else {

                    // reduce further
                    let a_ret = a.reduce(depth + 1);
                    let b_ret = b.reduce(depth + 1);

                    let mut left: Option<SnailfishNumber> = None;
                    let mut right: Option<SnailfishNumber> = None;

                    // Process reduction return values
                    match a_ret {
                        Explode(x, y) => {
                            // left exploded.
                            // add fragments

                        }
                        LeftFragment(frag) => {
                            // Something below exploded. Handle fragment
                        }
                        RightFragment(num) => {
                            // Something below exploded. Handle fragment
                        }
                        _ => { left = Some(a_ret); }
                    }

                    match b_ret {
                        Explode(a, b) => {
                            // right exploded.
                            Number(0);
                        }
                        _ => { right = Some(b_ret); }
                    }


                    Number(0)
                }
            }
            Number(a) => {
                if a >= 10 {
                    // Split Snailfish Number
                    let x = a as f32 / 2_f32;

                    Pair(Box::new(Number(x.floor() as u32)),
                         Box::new(Number(x.ceil() as u32)))
                } else {
                    // Nothing to do, cannot reduce. Return self
                    Number(a)
                }
            }
            Explode(_, _) => {
                panic!("Error: called reduce on an exploding Snailfish Number");
            }
            LeftFragment(a, b) => {
                panic!("Error: called reduce on a Snailfish Number fragment");
            }
            RightFragment(a, b) => {
                panic!("Error: called reduce on a Snailfish Number fragment");
            }
        }

        //
        //
        // if let Pair(x, y) = self {
        //     if depth > 4 {
        //         // Explode Snailfish Number
        //         // Assumption: Exploding pairs will always consist of two regular numbers.
        //         let a = x.unwrap();
        //         let b = y.unwrap();
        //
        //         Explode(a, b)
        //     } else {
        //         // reduce further
        //         let x_ret = x.reduce(depth+1);
        //         let y_ret = y.reduce(depth+1);
        //
        //         match x_ret {
        //             Explode(a, b) => {Number(0)},
        //             Number(a) => {Number(0)},
        //             Pair(a, b) => {Number(0)}
        //         }
        //
        //     }
        //
        // } else if let Number(x) = self {
        //     if x >= 10 {
        //         // Split Snailfish Number
        //         let a = x as f32 / 2_f32;
        //
        //         Pair(Box::new(Number(a.floor() as u32)),
        //              Box::new(Number(a.ceil() as u32)))
        //     } else {
        //         // Do nothing, return self
        //         Number(x)
        //     }
        // } else {
        //     panic!("Error: Snailfish Number couldn't be reduced")
        // }
    }
}


fn parse_input(file_path: &str) -> Vec<SnailfishNumber> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<SnailfishNumber>::new();

    for line in contents.lines() {
        result.push(parse_snailfish_number(line));
    }

    result
}


fn parse_snailfish_number(input: &str) -> SnailfishNumber {
    if input.starts_with('[') && input.ends_with(']') {
        // find left and right and return Pair
        let (left_num, right_num) = split_snailfish_interior(&input[1..input.len() - 1]);

        Pair(Box::new(parse_snailfish_number(left_num)),
             Box::new(parse_snailfish_number(right_num)))
    } else if input.len() == 1 && input.chars().next().unwrap().is_numeric() {
        // found a Number, not a Pair

        Number(input.chars().next().unwrap().to_digit(10).unwrap())
    } else {
        panic!("Error: invalid Snailfish number, input = {}", input);
    }
}

fn split_snailfish_interior(input: &str) -> (&str, &str) {
    if input.is_empty() {
        return ("", "");
    }

    let mut balance = 0_usize;
    let mut slice_index = 0_usize;

    for (i, c) in input.iter_elements().enumerate() {
        if c == '[' {
            balance += 1;
        } else if c == ']' {
            balance -= 1;
        } else if c == ',' && balance == 0 {
            slice_index = i;
            break;
        }
    }

    (&input[0..slice_index], &input[slice_index + 1..])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(0, solve_first_star());
        assert_eq!(0, solve_second_star());
    }

    #[test]
    fn splitting_snailfish_interior() {
        assert_eq!(("", ""), split_snailfish_interior(""));
        assert_eq!(("2", "6"), split_snailfish_interior("2,6"));

        assert_eq!(("[4,6]", "[[2,2],[3,0]]"), split_snailfish_interior("[4,6],[[2,2],[3,0]]"));
    }

    #[test]
    fn parsing_snailfish_number() {
        assert_eq!(
            Pair(Box::from(Number(2)), Box::from(Number(6))),
            parse_snailfish_number("[2,6]"));
    }

    #[test]
    fn snailfish_splitting() {
        assert_eq!(
            Pair(Box::from(Number(5)), Box::from(Number(6))),
            Number(11).reduce(0));

        assert_eq!(
            Pair(Box::from(Number(5)), Box::from(Number(5))),
            Number(10).reduce(0));

        assert_eq!(
            Pair(Box::from(Number(6)), Box::from(Number(6))),
            Number(12).reduce(0));
    }
}