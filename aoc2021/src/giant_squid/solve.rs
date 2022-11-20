use std::collections::VecDeque;
use std::fs;
use ndarray::{Array2};


pub fn solve_first_star() -> u32 {
    let (bingo_results, mut bingo_boards) = parse_input("src/giant_squid/input.txt");

    for bingo_result in bingo_results {

        // score result across all boards
        for board in bingo_boards.iter_mut() {
            score_result(board, bingo_result);

            if check_win(board) {
                // Found a winner, filter data with unmarked results (i.e. marks == 0)
                board.data.zip_mut_with(&board.marks, |x, y| { *x *= 1 - *y });

                let board_sum = board.data.sum();

                // calculate return result
                return bingo_result * board_sum;
            }
        }

    }

    0
}


pub fn solve_second_star() -> u32 {
    let (bingo_results, mut bingo_boards) = parse_input("src/giant_squid/input.txt");

    let mut last_winner = 0;

    let mut purge_list:Vec<usize> = vec![];

    for bingo_result in bingo_results {

        // purge boards from last loop starting with highest index first
        for index in purge_list.iter().rev() {
            bingo_boards.remove(*index);
        }

        // clear down purge list ready for next result processing
        purge_list.clear();

        // score result across all boards
        for (index, board) in bingo_boards.iter_mut().enumerate() {
            score_result(board, bingo_result);

            if check_win(board) {
                // Found a winner, filter data with unmarked results (i.e. marks == 0)
                board.data.zip_mut_with(&board.marks, |x, y| { *x *= 1 - *y });

                let board_sum = board.data.sum();

                // save latest winning result
                last_winner = bingo_result * board_sum;

                // add board to purge list
                purge_list.push(index);

            }
        }

    }

    last_winner
}


fn score_result(bingo_board: &mut BingoBoard, bingo_result: u32) {
    for row_index in 0..bingo_board.data.len_of(ndarray::Axis(0)) {
        let row = bingo_board.data.row(row_index);
        let result = row.iter().position(|x| *x == bingo_result);

        if let Some(column_index) = result {
            bingo_board.marks[[row_index, column_index]] = 1;
        }
    }
}

fn check_win(bingo_board: &BingoBoard) -> bool {
    for col in bingo_board.marks.columns() {
        if col.sum() == 5 {
            // Winner
            return true;
        }
    }
    for row in bingo_board.marks.rows() {
        if row.sum() == 5 {
            // Winner
            return true;
        }
    }

    false
}


// Struct to manage Bingo Board data and marking
struct BingoBoard {
    data: Array2<u32>,
    marks: Array2<u32>,
}

fn parse_input(file_path: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");


    let bingo_data_vec = contents
        .lines()
        .collect::<Vec<_>>();

    let mut bingo_data_deque = VecDeque::from(bingo_data_vec);

    // parse bingo results (first entry)
    let bingo_results = bingo_data_deque
        .pop_front().unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // parse bingo boards
    let mut bingo_boards = vec![];

    while let Some(data) = bingo_data_deque.pop_front() {
        if data == "" {
            // New board. Lets set it up.
            let mut new_board = BingoBoard {
                data: Array2::<u32>::zeros((5, 5)),
                marks: Array2::<u32>::zeros((5, 5)),
            };

            for mut row in new_board.data.rows_mut() {
                let row_data = bingo_data_deque
                    .pop_front().unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                row[0] = row_data[0];
                row[1] = row_data[1];
                row[2] = row_data[2];
                row[3] = row_data[3];
                row[4] = row_data[4];
            }

            // move board into bingo boards result
            bingo_boards.push(new_board);
        }
    }

    (bingo_results, bingo_boards)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(10374, solve_first_star());
        assert_eq!(24742, solve_second_star());
    }
}