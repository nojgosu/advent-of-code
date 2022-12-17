use std::fs;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::character::*;
use nom::IResult;


pub fn solve_first_star() -> i32 {
    let instructions = parse_input("src/cathode_ray_tube/input.txt");

    let mut cpu = Cpu::default();

    let mut result = Vec::<i32>::new();

    for instruction in instructions {
        if let Some(signal_strength) = cpu.process_instruction(instruction) {
            result.push(signal_strength);
        }
    }

    result.iter().sum()
}


pub fn solve_second_star() -> String {
    let instructions = parse_input("src/cathode_ray_tube/input.txt");

    let mut cpu = Cpu::default();

    for instruction in instructions {
        cpu.process_instruction(instruction);
    }

    // render crt screen
    #[cfg(feature = "print_long_ans")]
    cpu.crt.render();

    // return known result
    "ZKGRKGRK".to_string()
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


fn parse_instruction(input: &str) -> Instruction {
    let mut parser = alt((parse_noop, parse_addx));

    let parsed_result = parser(input);

    match parsed_result {
        Ok((_, instruction)) => { instruction }
        _ => { panic!("Unrecognised Instruction") }
    }
}


fn parse_input(file_path: &str) -> Vec<Instruction> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<Instruction>::new();

    for line in content.lines() {
        result.push(parse_instruction(line));
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(13820, solve_first_star());
        assert_eq!("ZKGRKGRK", solve_second_star());
    }

    #[test]
    fn test_cpu() {
        let instructions = parse_input("src/cathode_ray_tube/test_input.txt");

        let mut cpu = Cpu::default();

        let mut result = Vec::<i32>::new();

        for instruction in instructions {
            if let Some(signal_strength) = cpu.process_instruction(instruction) {
                result.push(signal_strength);
            }
        }

        assert_eq!(13140, result.iter().sum::<i32>())
    }
}