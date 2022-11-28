use std::collections::VecDeque;
use std::fs;
use ndarray::{Array2, Axis, s};


pub fn solve_first_star() -> u64 {
    let (paper, instructions) = parse_input("src/transparent_origami/input.txt");

    let folded_paper = fold_paper(paper, *instructions.first().unwrap());

    folded_paper.iter().filter(|&x| *x).count() as u64
}


pub fn solve_second_star() -> &'static str {
    let (paper, instructions) = parse_input("src/transparent_origami/input.txt");

    let mut folded_paper = paper;

    for instruction in instructions {
        folded_paper = fold_paper(folded_paper, instruction);
    }

    // To declutter printed output, disable behind feature flag
    #[cfg(feature = "print_long_ans")]
    println!("Origami = \n{:?}", folded_paper.map(|x| if *x {'#'} else {'.'}));

    // reviewing the printed output from the folded_paper we get the following code
    "FPEKBEJL"
}

fn fold_paper(paper: Array2<bool>, instruction: (char, usize)) -> Array2<bool> {
    match instruction.0 {
        'x' => {

            let fold_col = instruction.1;

            // divide paper into two halves along the fold crease
            let mut left_side = paper.slice(s![.., ..fold_col]).to_owned();
            let mut right_side = paper.slice(s![.., fold_col+1..]).to_owned();

            // invert column axis to account for mirroring of right side when folded
            right_side.invert_axis(Axis(1));

            // 'fold' pages together and return new folded page
            left_side.zip_mut_with(&right_side, |a_elem, b_elem| { *a_elem = *a_elem || *b_elem });

            left_side
        }
        'y' => {

            let fold_row = instruction.1;

            // divide paper into two halves along the fold crease
            let mut top_side = paper.slice(s![..fold_row, ..]).to_owned();
            let mut bottom_side = paper.slice(s![fold_row+1.., ..]).to_owned();

            // invert column axis to account for mirroring of right side when folded
            bottom_side.invert_axis(Axis(0));

            // 'fold' pages together and return new folded page
            top_side.zip_mut_with(&bottom_side, |a_elem, b_elem| { *a_elem = *a_elem || *b_elem });

            top_side
        }
        _ => { panic!("Error in folding instructions") }
    }
}


fn parse_input(file_path: &str) -> (Array2<bool>, Vec<(char, usize)>) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut lines = contents.lines().collect::<VecDeque<_>>();

    let mut points = Vec::<(usize, usize)>::new();

    // process points
    while let Some(input) = lines.pop_front() {
        match input {
            "" => { break; }
            _ => {
                // add point to paper
                let mut point_location = input.split(',');

                let col = point_location.next().unwrap().parse::<usize>().unwrap();
                let row = point_location.next().unwrap().parse::<usize>().unwrap();

                points.push((row, col));
            }
        }
    }

    // construct fold instructions
    let mut instructions = Vec::<(char, usize)>::new();

    while let Some(input) = lines.pop_front() {
        let mut fold_text = input.split('=');

        let fold_axis = fold_text.next().unwrap().chars().last().unwrap();

        let fold_line = fold_text.next().unwrap().parse::<usize>().unwrap();

        instructions.push((fold_axis, fold_line));
    }

    // calculate paper size from first x and y fold instruction
    let (_, first_x_fold) = instructions
        .iter().find(|(axis, _)| { *axis == 'x' })
        .unwrap();

    let (_, first_y_fold) = instructions
        .iter().find(|(axis, _)| { *axis == 'y' })
        .unwrap();

    let paper_width = first_x_fold * 2 + 1;

    let paper_height = first_y_fold * 2 + 1;

    // construct origami paper from points
    let mut paper = Array2::<bool>::from_elem((paper_height, paper_width), false);

    for (row, col) in points {
        paper[[row, col]] = true;
    }

    (paper, instructions)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(701, solve_first_star());
        assert_eq!("FPEKBEJL", solve_second_star());
    }
}