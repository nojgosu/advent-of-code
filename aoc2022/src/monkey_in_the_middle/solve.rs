use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::rc::Rc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::character::*;
use nom::combinator::rest;
use nom::IResult;


pub fn solve_first_star() -> usize {
    //let monkeys = parse_input("src/monkey_in_the_middle/input.txt");

    let mut monkeys = init_monkeys();

    // cycles
    for _ in 0..10000 {
        for monkey in monkeys.iter() {
            let tossed_items = monkey.borrow_mut().rummage();

            for (item, dest) in tossed_items {
                // assign items
                monkeys[dest].borrow_mut().items.push_back(item);
            }
        }
    }


    // 112221 correct answer
    monkeys.sort();

    //dbg!(&monkeys);

    let ans = monkeys.pop().unwrap().borrow_mut().inspection_count *
        monkeys.pop().unwrap().borrow_mut().inspection_count;


    // part 2
    // 19335041548 too low
    // 19337822658 too low
    // 20069963890 too low
    // 14402520108 too low (didn't submit)
    // 14402880128 too low (didn't submit)
    // 25272176808 (correct answer)

    ans
}


pub fn solve_second_star() -> usize {
    //let instructions = parse_input("src/monkey_in_the_middle/input.txt");

    // Trick
    // As far as I know, there are no general shortcuts besides reducing the
    // terms of the product before multiplying. I will emphasize that by reduction,
    // I mean picking the smallest representative mod m in absolute value.
    // This may be a negative integer.

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem

    // modular multiplication

    // https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-multiplication
    //

    //




    0
}


fn init_monkeys() -> Vec<RefCell<Monkey>> {
    let mut result = Vec::new();

    // Monkey 0
    let m = Monkey {
        items: VecDeque::from(vec![54, 98, 50, 94, 69, 62, 53, 85]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 3) * (13 % 3)),
        operation: Box::new(|old| old * 13),
        test: Box::new(|x| x % 3),
        test_true: 2,
        test_false: 1,
    };

    result.push(RefCell::new(m));

    // Monkey 1
    let m = Monkey {
        items: VecDeque::from(vec![71, 55, 82]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 13) + (2 % 13)),
        operation: Box::new(|old| old + 2),
        test: Box::new(|x| x % 13),
        test_true: 7,
        test_false: 2,
    };

    result.push(RefCell::new(m));

    // Monkey 2
    let m = Monkey {
        items: VecDeque::from(vec![77, 73, 86, 72, 87]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 19) + (8 % 19)),
        operation: Box::new(|old| old + 8),
        test: Box::new(|x| x % 19),
        test_true: 4,
        test_false: 7,
    };

    result.push(RefCell::new(m));

    // Monkey 3
    let m = Monkey {
        items: VecDeque::from(vec![97, 91]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 17) + (1 % 17)),
        operation: Box::new(|old| old + 1),
        test: Box::new(|x| x % 17),
        test_true: 6,
        test_false: 5,
    };

    result.push(RefCell::new(m));

    // Monkey 4
    let m = Monkey {
        items: VecDeque::from(vec![78, 97, 51, 85, 66, 63, 62]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 5) * (17 % 5)),
        operation: Box::new(|old| old * 17),
        test: Box::new(|x| x % 5),
        test_true: 6,
        test_false: 3,
    };

    result.push(RefCell::new(m));

    // Monkey 5
    let m = Monkey {
        items: VecDeque::from(vec![88]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 7) + (3 % 7)),
        operation: Box::new(|old| old + 3),
        test: Box::new(|x| x % 7),
        test_true: 1,
        test_false: 0,
    };

    result.push(RefCell::new(m));

    // Monkey 6
    let m = Monkey {
        items: VecDeque::from(vec![87, 57, 63, 86, 87, 53]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 11) * (old % 11)),
        operation: Box::new(|old| old * old),
        test: Box::new(|x| x % 11),
        test_true: 5,
        test_false: 0,
    };

    result.push(RefCell::new(m));

    // Monkey 7
    let m = Monkey {
        items: VecDeque::from(vec![73, 59, 82, 65]),
        inspection_count: 0,
        //operation: Box::new(|old| (old % 2) + (6 % 2)),
        operation: Box::new(|old| old + 6),
        test: Box::new(|x| x % 2),
        test_true: 4,
        test_false: 3,
    };

    result.push(RefCell::new(m));


    result
}


struct Monkey {
    items: VecDeque<u64>,
    inspection_count: usize,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> u64>,
    test_true: usize,
    test_false: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspection_count", &self.inspection_count)
            .field("test_true", &self.test_true)
            .field("test_false", &self.test_false)
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
    fn inspect_item(&mut self) -> Option<(u64, usize)> {
        if let Some(item) = self.items.pop_front() {
            self.inspection_count += 1;

            let worry_level = (self.operation)(item);
            let worry_level = worry_level % 9699690;

            if (self.test)(worry_level) == 0 {
                Some((worry_level, self.test_true))
            } else {
                Some((worry_level, self.test_false))
            }
        } else {
            None
        }
    }

    fn rummage(&mut self) -> Vec<(u64, usize)> {
        let mut result = Vec::new();

        while !self.items.is_empty() {
            if let Some(tossed_item) = self.inspect_item() {
                result.push(tossed_item);
            }
        }

        result
    }
}


#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, PartialEq, Eq)]
struct Cpu {
    cycle: usize,
    regx: i32,
    crt: Crt,
}

#[derive(Debug, PartialEq, Eq)]
struct Crt {
    screen: [char; 240],
}

impl Cpu {
    fn default() -> Self {
        Cpu {
            cycle: 0,
            regx: 1,
            crt: Crt::default(),
        }
    }

    fn process_instruction(&mut self, instr: Instruction) -> Option<i32> {
        let mut result: Option<i32> = None;

        match instr {
            Instruction::Noop => {
                self.cycle += 1;
                self.crt.write_pixel(self.cycle, self.regx);
                result = self.emit_signal_strength()
            }
            Instruction::Addx(val) => {
                self.cycle += 1;
                self.crt.write_pixel(self.cycle, self.regx);
                result = self.emit_signal_strength();
                self.cycle += 1;
                self.crt.write_pixel(self.cycle, self.regx);
                if result.is_none() {
                    result = self.emit_signal_strength();
                }
                self.regx += val;
            }
        }

        result
    }

    fn emit_signal_strength(&self) -> Option<i32> {
        if self.cycle < 20 {
            None
        } else if (self.cycle - 20) % 40 == 0 {
            // emit signal strength
            Some(self.cycle as i32 * self.regx)
        } else {
            None
        }
    }
}


impl Crt {
    fn default() -> Self {
        Crt {
            screen: ['.'; 240]
        }
    }

    fn write_pixel(&mut self, cycle: usize, regex: i32) {
        let sprite = [regex - 1, regex, regex + 1];

        // normalise crt x pos
        let crt_x = (cycle - 1) % 40;

        if sprite.contains(&(crt_x as i32)) {
            self.screen[cycle - 1] = '#';
        } else {
            self.screen[cycle - 1] = '.';
        }
    }

    fn render(&self) {
        for i in 0..240 {
            if i % 40 == 0 {
                print!("\n");
            }
            print!("{}", self.screen[i]);
        }
    }
}


fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("noop")(rest)?;
    let (rest, _) = space0(rest)?;

    Ok((rest, Instruction::Noop))
}


fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("addx")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, val) = complete::i32(rest)?;

    Ok((rest, Instruction::Addx(val)))
}

fn parse_items(input: &str) -> IResult<&str, Vec<u32>> {
    let result = Vec::<u32>::new();

    let (rest, _) = space0(input)?;
    let (rest, _) = tag("addx")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, val) = complete::i32(rest)?;

    Ok((rest, result))
}

fn parse_item(input: &str) -> IResult<&str, u32> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("addx")(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, val) = complete::i32(rest)?;

    Ok((rest, 0))
}


fn parse_instruction(input: &str) -> Instruction {
    let mut parser = alt((parse_noop, parse_addx));

    let parsed_result = parser(input);

    match parsed_result {
        Ok((_, instruction)) => { instruction }
        _ => { panic!("Unrecognised Instruction") }
    }
}


fn parse_input(file_path: &str) -> Vec<Monkey> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<Monkey>::new();

    println!("{:?}", content);

    // parse until Monkey (left parse struct, right continue)

    for line in content.lines() {
        //result.push(parse_instruction(line));
    }

    result
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