use std::fs;
use std::ops::Index;


pub fn solve_first_star() -> u64 {
    let replies = parse_input("src/syntax_scoring/input.txt");


    let (error_score, _) = parse_syntax_scoring(replies);

    error_score
}

pub fn solve_second_star() -> u64 {
    let replies = parse_input("src/syntax_scoring/input.txt");


    let (_, incomplete_score) = parse_syntax_scoring(replies);

    incomplete_score
}


fn parse_syntax_scoring(replies: Vec<String>) -> (u64, u64) {
    let mut square_errors = 0;
    let mut parentheses_errors = 0;
    let mut curly_errors = 0;
    let mut angle_errors = 0;
    let mut incomplete_scores = Vec::<u64>::new();

    for reply in replies {
        let mut incomplete_score = 0u64;

        // flag to track if reply is corrupted
        let mut corrupted = false;

        let mut tokens = Vec::<char>::new();

        for c in reply.chars() {
            if c == '[' || c == '{' || c == '<' || c == '(' {
                tokens.push(c);
            } else {
                let token = tokens.pop();

                if let Some(terminator) = token {
                    // incorrect closing bracket for chunk
                    match c {
                        ']' => {
                            if terminator != '[' {
                                square_errors += 1;
                                corrupted = true
                            }
                        }
                        ')' => {
                            if terminator != '(' {
                                parentheses_errors += 1;
                                corrupted = true
                            }
                        }
                        '}' => {
                            if terminator != '{' {
                                curly_errors += 1;
                                corrupted = true
                            }
                        }
                        '>' => {
                            if terminator != '<' {
                                angle_errors += 1;
                                corrupted = true
                            }
                        }
                        _ => { panic!("Error, unknown input in tokens") }
                    }
                } else {
                    // Closing bracket with no opening bracket - Not encountered in input.txt
                    match c {
                        ']' => square_errors += 1,
                        ')' => parentheses_errors += 1,
                        '}' => curly_errors += 1,
                        '>' => angle_errors += 1,
                        _ => { panic!("Error, unknown input in tokens") }
                    }
                }
            }
        }

        // calculate incomplete score using left over tokens only if reply is not corrupted
        if !corrupted {
            while let Some(leftover) = tokens.pop() {
                match leftover {
                    '(' => incomplete_score = incomplete_score * 5 + 1,
                    '[' => incomplete_score = incomplete_score * 5 + 2,
                    '{' => incomplete_score = incomplete_score * 5 + 3,
                    '<' => incomplete_score = incomplete_score * 5 + 4,
                    _ => { panic!("Error, unknown input in tokens") }
                }
            }

            // add incomplete_score to list of scores
            incomplete_scores.push(incomplete_score);
        }
    }

    // Calculate syntax error score
    let parentheses_error_multiplier = 3;
    let square_error_multiplier = 57;
    let curly_error_multiplier = 1197;
    let angle_error_multiplier = 25137;

    let error_score = parentheses_errors * parentheses_error_multiplier +
        square_errors * square_error_multiplier +
        curly_errors * curly_error_multiplier +
        angle_errors * angle_error_multiplier;

    // Sort incomplete sorts so the median can be returned
    incomplete_scores.sort();

    (error_score, *incomplete_scores.index(incomplete_scores.len() / 2))
}


fn parse_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    contents.lines().map(String::from).collect::<Vec<_>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(311895, solve_first_star());
        assert_eq!(2904180541, solve_second_star());
    }
}