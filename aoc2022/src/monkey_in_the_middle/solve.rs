use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::fs;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, newline, one_of, space0, space1};
use nom::character::*;
use nom::error::Error;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;


pub fn solve_first_star() -> usize {
    let content = parse_input("src/monkey_in_the_middle/input.txt");

    let mut monkeys;

    if let Ok((_, m)) = separated_list1(line_ending, parse_monkey)(&content) {
        monkeys = m;
    } else {
        panic!("Error: couldn't parse monkeys")
    }

    let relief = Box::new(|x: u64| x / 3);

    // cycles
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let tossed_items = monkey.borrow_mut().rummage(relief.clone());

            for (item, dest) in tossed_items {
                // assign items
                monkeys[dest].borrow_mut().items.push_back(item);
            }
        }
    }

    monkeys.sort();


    let ans = monkeys.pop().unwrap().borrow_mut().inspection_count *
        monkeys.pop().unwrap().borrow_mut().inspection_count;

    ans
}


pub fn solve_second_star() -> usize {
    let content = parse_input("src/monkey_in_the_middle/input.txt");

    let mut monkeys;

    if let Ok((_, m)) = separated_list1(line_ending, parse_monkey)(&content) {
        monkeys = m;
    } else {
        panic!("Error: couldn't parse monkeys")
    }

    let relief = Box::new(|x: u64| x % 9699690);

    // cycles
    for _ in 0..10000 {
        for monkey in monkeys.iter() {
            let tossed_items = monkey.borrow_mut().rummage(relief.clone());

            for (item, dest) in tossed_items {
                // assign items
                monkeys[dest].borrow_mut().items.push_back(item);
            }
        }
    }


    // 112221 correct answer
    monkeys.sort();

    let ans = monkeys.pop().unwrap().borrow_mut().inspection_count *
        monkeys.pop().unwrap().borrow_mut().inspection_count;

    ans
}


struct Monkey {
    items: VecDeque<u64>,
    inspection_count: usize,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> u64>,
    true_dest: usize,
    false_dest: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspection_count", &self.inspection_count)
            .field("test_true", &self.true_dest)
            .field("test_false", &self.false_dest)
            .finish()
    }
}

impl Eq for Monkey {}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inspection_count.cmp(&other.inspection_count)
    }
}


impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.inspection_count.cmp(&other.inspection_count))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.inspection_count == other.inspection_count
    }
}

impl Monkey {
    fn inspect_item<F>(&mut self, relief: F) -> Option<(u64, usize)> where
        F: Fn(u64) -> u64 {
        if let Some(item) = self.items.pop_front() {
            self.inspection_count += 1;

            let worry_level = (self.operation)(item);
            //let worry_level = worry_level % 9699690;

            let relieved_worry = relief(worry_level);

            if (self.test)(relieved_worry) == 0 {
                Some((relieved_worry, self.true_dest))
            } else {
                Some((relieved_worry, self.false_dest))
            }
        } else {
            None
        }
    }

    fn rummage<F>(&mut self, relief: F) -> Vec<(u64, usize)> where
        F: Fn(u64) -> u64 {
        let mut result = Vec::new();

        while !self.items.is_empty() {
            if let Some(tossed_item) = self.inspect_item(relief.borrow()) {
                result.push(tossed_item);
            }
        }

        result
    }
}


fn parse_monkey(input: &str) -> IResult<&str, RefCell<Monkey>> {
    // Sample input:
    // Monkey 0:
    //      Starting items: 79, 98
    //      Operation: new = old * 19
    //      Test: divisible by 23
    //          If true: throw to monkey 2
    //          If false: throw to monkey 3

    let (rest, _) = tuple((space0, tag("Monkey "), digit1, tag(":"), line_ending))(input)?;
    let (rest, items) = parse_starting_items(rest)?;
    let (rest, operation) = parse_operation(rest)?;
    let (rest, test) = parse_divisible_test(rest)?;
    let (rest, true_dest) = parse_test_if_true(rest)?;
    let (rest, false_dest) = parse_test_if_false(rest)?;

    let m = Monkey {
        items,
        operation,
        test,
        true_dest,
        false_dest,
        inspection_count: 0,
    };

    Ok((rest, RefCell::new(m)))
}

fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    // Starting items: 79, 98
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("Starting items: ")(rest)?;
    let (rest, list) = separated_list1(tag(", "), complete::u64)(rest)?;
    let (rest, _) = line_ending(rest)?;

    Ok((rest, VecDeque::from(list)))
}

fn parse_operation(input: &str) -> IResult<&str, Box<dyn Fn(u64) -> u64>> {
    // Operation: new = old * 19
    // Operation: new = old + 6
    // Operation: new = old * old
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("Operation: new = old ")(rest)?;
    let (rest, op) = one_of("*+")(rest)?;
    let (rest, _) = space0(rest)?;
    let (rest, term) = alt((tag("old"), digit1))(rest)?;
    let (rest, _) = line_ending(rest)?;

    let result: Box<dyn Fn(u64) -> u64>;

    if term == "old" {
        match op {
            '+' => { result = Box::new(|old| old + old) }
            '*' => { result = Box::new(|old| old * old) }
            _ => { panic!("unknown operator") }
        }
    } else if let Ok((_, num)) = complete::u64::<_, Error<_>>(term) {
        match op {
            '+' => { result = Box::new(move |old| old + num) }
            '*' => { result = Box::new(move |old| old * num) }
            _ => { panic!("unknown operator") }
        }
    } else {
        panic!("Couldn't parse term: {}", term);
    }

    Ok((rest, result))
}

fn parse_divisible_test(input: &str) -> IResult<&str, Box<dyn Fn(u64) -> u64>> {
    //      Test: divisible by 23
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("Test: divisible by ")(rest)?;
    let (rest, num) = digit1(rest)?;
    let (rest, _) = line_ending(rest)?;

    let result: Box<dyn Fn(u64) -> u64>;

    if let Ok((_, num)) = complete::u64::<_, Error<_>>(num) {
        result = Box::new(move |x| x % num)
    } else {
        panic!("Error parsing divisible test number: {}", num);
    }

    Ok((rest, result))
}

fn parse_test_if_true(input: &str) -> IResult<&str, usize> {
    //          If true: throw to monkey 2
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("If true: throw to monkey ")(rest)?;
    let (rest, num) = digit1(rest)?;
    let (rest, _) = line_ending(rest)?;

    let result;

    if let Ok((_, num)) = complete::u64::<_, Error<_>>(num) {
        result = num as usize;
    } else {
        panic!("Error: Couldn't parse test if true num {}", num);
    }

    Ok((rest, result))
}

fn parse_test_if_false(input: &str) -> IResult<&str, usize> {
    //          If false: throw to monkey 3
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("If false: throw to monkey ")(rest)?;
    let (rest, num) = digit1(rest)?;
    let (rest, _) = line_ending(rest)?;

    let result;

    if let Ok((_, num)) = complete::u64::<_, Error<_>>(num) {
        result = num as usize;
    } else {
        panic!("Error: Couldn't parse test if false num {}", num);
    }

    Ok((rest, result))
}


fn parse_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Input file local to project")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(112221, solve_first_star());
        assert_eq!(25272176808, solve_second_star());
    }
}