use std::fs;
use std::str::FromStr;


pub fn solve_first_star() -> u64 {
    let plays = parse_input("src/rock_paper_scissors/input.txt");

    let mut score = 0u64;

    for play in plays {
        match play {
            ('A', 'X') => { score += 1 + 3 }    // Rock Rock Draw
            ('A', 'Y') => { score += 2 + 6 }    // Rock Paper Win
            ('A', 'Z') => { score += 3 }        // Rock Scissors Loss
            ('B', 'X') => { score += 1 }        // Paper Rock Loss
            ('B', 'Y') => { score += 2 + 3 }    // Paper Paper Draw
            ('B', 'Z') => { score += 3 + 6 }    // Paper Scissors Win
            ('C', 'X') => { score += 1 + 6 }    // Scissors Rock Win
            ('C', 'Y') => { score += 2 }        // Scissors Paper Loss
            ('C', 'Z') => { score += 3 + 3 }    // Scissors Scissors Draw
            play => { panic!("Error in input. Unrecognised play combination {:?}", play); }
        }
    }

    score
}


pub fn solve_second_star() -> u64 {
    let plays = parse_input("src/rock_paper_scissors/input.txt");

    let mut score = 0u64;

    for play in plays {
        match play {
            ('A', 'X') => { score += 3 }        // Rock & Need to loose. Pick Scissors
            ('A', 'Y') => { score += 1 + 3 }    // Rock & Need to draw. Pick Rock
            ('A', 'Z') => { score += 2 + 6 }    // Rock & Need to win. Pick Paper
            ('B', 'X') => { score += 1 }        // Paper & Need to loose. Pick Rock
            ('B', 'Y') => { score += 2 + 3 }    // Paper & Need to draw. Pick Paper
            ('B', 'Z') => { score += 3 + 6 }    // Paper & Need to win. Pick Scissors
            ('C', 'X') => { score += 2 }        // Scissors & Need to loose. Pick Paper
            ('C', 'Y') => { score += 3 + 3 }    // Scissors & Need to draw. Pick Scissors
            ('C', 'Z') => { score += 1 + 6 }    // Scissors & Need to win. Pick Rock
            play => { panic!("Error in input. Unrecognised play combination {:?}", play); }
        }
    }

    score
}


fn parse_input(file_path: &str) -> Vec<(char, char)> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<(char, char)>::new();

    for line in contents.lines() {
        let mut symbols = line.split(' ');

        let c1 = char::from_str(symbols.next().unwrap()).unwrap();

        let c2 = char::from_str(symbols.next().unwrap()).unwrap();


        result.push((c1, c2));
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(14531, solve_first_star());
        assert_eq!(11258, solve_second_star());
    }
}