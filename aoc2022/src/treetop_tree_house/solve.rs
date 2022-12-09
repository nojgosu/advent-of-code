use std::fs;
use ndarray::{Array1, Array2, ArrayView, Axis, Ix1, Zip};


pub fn solve_first_star() -> usize {
    let forest = parse_input("src/treetop_tree_house/input.txt");

    let visible_trees = find_visible_trees(&forest);

    visible_trees.iter().filter(|&a| *a).count()
}


pub fn solve_second_star() -> u32 {
    let forest = parse_input("src/treetop_tree_house/input.txt");

    let scenic_forest = survey_forest(forest);

    scenic_forest.into_iter().max().unwrap()
}


fn find_visible_trees(forest: &Array2<u32>) -> Array2<bool> {
    let (height, width) = forest.dim();

    let mut visible_rows = Array2::from_elem((0, width), false);
    let mut visible_cols = Array2::from_elem((height, 0), false);

    // build array of visible trees from row traversal
    for row in forest.rows() {
        visible_rows.push(Axis(0), visible_tree_slice(&row).view()).unwrap();
    }

    // build array of visible trees from row traversal
    for col in forest.columns() {
        visible_cols.push(Axis(1), visible_tree_slice(&col).view()).unwrap();
    }

    // Zip visible rows and visible cols together to create map of all visible trees
    Zip::from(&mut visible_rows).and(&visible_cols).for_each(|a, &b| *a = *a || b);

    visible_rows
}


fn visible_tree_slice(trees: &ArrayView<u32, Ix1>) -> Array1<bool> {
    let length = trees.dim();

    let mut result = Array1::from_elem(length, false);

    // traverse forwards
    let mut tallest_tree = trees[0];
    for index in 1..trees.len() {
        if trees[[index]] > tallest_tree {
            result[[index]] = true;
            tallest_tree = trees[[index]];
        }
    }

    // traverse backwards
    let mut tallest_tree = trees[length - 1];
    for index in 1..trees.len() {
        if trees[[length - index - 1]] > tallest_tree {
            result[[length - index - 1]] = true;
            tallest_tree = trees[[length - index - 1]];
        }
    }

    // set edges as visible
    result[[0]] = true;
    result[[trees.len() - 1]] = true;

    result
}


fn survey_forest(trees: Array2<u32>) -> Array2<u32> {
    let dim = trees.dim();

    let mut result = Array2::<u32>::zeros(dim);

    for ((r, c), _) in trees.indexed_iter() {
        result[[r, c]] = scenic_score(&trees, (r, c));
    }

    result
}


fn scenic_score(trees: &Array2<u32>, position: (usize, usize)) -> u32 {
    // initialise as 1 because its a product
    let mut scenic_score = 1;

    // Look all directions
    scenic_score *= gaze_direction(trees, position, (-1, 0)); // North
    scenic_score *= gaze_direction(trees, position, (1, 0)); // South
    scenic_score *= gaze_direction(trees, position, (0, -1)); // West
    scenic_score *= gaze_direction(trees, position, (0, 1)); // East

    scenic_score
}

fn gaze_direction(trees: &Array2<u32>, position: (usize, usize), direction: (i32, i32)) -> u32 {
    let (row_len, col_len) = trees.dim();
    let mut scenic_score = 0_u32;

    let tree_house_height = trees[[position.0, position.1]];
    let mut row_index = position.0 as i32;
    let mut col_index = position.1 as i32;


    loop {
        // update indices
        row_index += direction.0;
        col_index += direction.1;

        // check for out of bounds
        if row_index < 0 || col_index < 0 || row_index as usize == row_len || col_index as usize == col_len { break; }

        // check line of sight and accumulate scenic score
        let assessing_tree = trees[[row_index as usize, col_index as usize]];

        scenic_score += 1;

        if assessing_tree >= tree_house_height {
            break;
        }
    }

    scenic_score
}


fn parse_input(file_path: &str) -> Array2<u32> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    // get dimensions of forest grid
    let width = contents.lines().next().unwrap().len();

    let mut result = Array2::<u32>::zeros((0, width));

    let lines = contents.lines();

    for line in lines {
        let trees = line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        result.push(Axis(0), ArrayView::from(&trees)).unwrap();
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solved() {
        assert_eq!(1708, solve_first_star());
        assert_eq!(504000, solve_second_star());
    }


    #[test]
    fn visible_trees() {
        let forest = parse_input("src/treetop_tree_house/test_input.txt");
        let result = find_visible_trees(&forest);
        let ans = result.iter().filter(|&a| *a);
        assert_eq!(21, ans.count());
    }


    #[test]
    fn gazing() {
        let forest = parse_input("src/treetop_tree_house/test_input.txt");
        assert_eq!(2, gaze_direction(&forest, (2, 0), (1, 0)));   // South
        assert_eq!(0, gaze_direction(&forest, (0, 2), (-1, 0)));  // North
        assert_eq!(1, gaze_direction(&forest, (0, 2), (1, 0)));   // South
        assert_eq!(2, gaze_direction(&forest, (3, 2), (-1, 0)));  // North
        assert_eq!(1, gaze_direction(&forest, (3, 2), (1, 0)));   // South
        assert_eq!(2, gaze_direction(&forest, (3, 2), (0, -1)));  // West
        assert_eq!(2, gaze_direction(&forest, (3, 2), (0, 1)));   // East
    }


    #[test]
    fn highest_scenic_score() {
        let forest = parse_input("src/treetop_tree_house/test_input.txt");
        let scenic_forest = survey_forest(forest);
        let highest_score = scenic_forest.iter().max().unwrap();
        assert_eq!(8, *highest_score);
    }
}