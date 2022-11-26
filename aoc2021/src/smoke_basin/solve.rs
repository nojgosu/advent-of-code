use std::fs;
use ndarray::{Array2, ArrayView, Axis};


pub fn solve_first_star() -> u32 {
    let terrain = parse_input("src/smoke_basin/input.txt");

    let (_, low_point_values) = find_minima(&terrain);

    // calculate risk factor and return
    low_point_values.len() as u32 + low_point_values.iter().sum::<u32>()
}

pub fn solve_second_star() -> u32 {
    let terrain = parse_input("src/smoke_basin/input.txt");

    let (locations, _) = find_minima(&terrain);

    let mut basin_sizes = Vec::<u32>::new();

    for location in locations {
        basin_sizes.push(calculate_basin_size(&terrain, location));
    }

    // multiple 3 largest basin sizes and return result
    basin_sizes.sort();
    basin_sizes.reverse();

    let mut result = 1_u32;

    for i in 0..3 {
        result *= basin_sizes[i];
    }

    result
}

/// recursively visits nodes in the terrain, counting nodes until reaching 9 or edge of terrain
fn visit_nodes(terrain: &Array2<u32>, visited: &mut Array2<bool>, cell: (usize, usize)) -> u32 {
    let size = terrain.shape();
    let height = size[0];
    let width = size[1];

    let mut count = 0_u32;

    // check if already visited
    if visited[[cell.0, cell.1]] {
        return 0;
    }

    // get cell value
    let value = terrain[[cell.0, cell.1]];

    if value == 9_u32 {
        // Hit wall of basin. Don't explore further
        return 0;
    }

    // mark as visited and count value
    visited[[cell.0, cell.1]] = true;
    count += 1;

    // visit edge nodes, above, below, left and right.
    // generate vector of nodes to visit
    let mut edge_nodes = Vec::<(usize, usize)>::new();

    // Handle boundaries of terrain
    if cell.0 == 0 {
        // top row, no cell above
    } else {
        edge_nodes.push((cell.0 - 1, cell.1))
    }

    if cell.0 == height - 1 {
        // bottom row, no cell below
    } else {
        edge_nodes.push((cell.0 + 1, cell.1))
    }

    if cell.1 == 0 {
        // left column, no cell left
    } else {
        edge_nodes.push((cell.0, cell.1 - 1))
    }

    if cell.1 == width - 1 {
        // right column, no cell right
    } else {
        edge_nodes.push((cell.0, cell.1 + 1))
    }

    for edge_node  in edge_nodes {
        // visit edge node
        count += visit_nodes(terrain, visited, edge_node);
    }

    count
}

fn calculate_basin_size(terrain: &Array2<u32>, low_point: (usize, usize)) -> u32 {

    // create visited map to track visited nodes
    let mut visited = Array2::<bool>::from_elem(terrain.raw_dim(), false);

    // visit nodes recursively starting from low point searching for basin edge whilst counting size
    visit_nodes(terrain, &mut visited, low_point)

}

fn find_minima(terrain: &Array2<u32>) -> (Vec<(usize, usize)>, Vec<u32>) {
    let terrain_size = terrain.shape();

    let height = terrain_size[0];
    let width = terrain_size[1];

    let mut low_point_values = Vec::<u32>::new();
    let mut low_point_locations = Vec::<(usize, usize)>::new();

    for i in 0..height {
        for j in 0..width {
            // calculate if position (i,j) is a minima.
            let cell = terrain[[i, j]];

            let mut above = None;
            let mut below = None;
            let mut left = None;
            let mut right = None;
            let default = Some(10u32); // default value that will always be a maxima

            // Handle boundaries of terrain
            if i == 0 {
                // top row, no cell above
                above = None;
            } else {
                above = Some(terrain[[i - 1, j]]);
            }

            if i == height - 1 {
                // bottom row, no cell below
                below = None;
            } else {
                below = Some(terrain[[i + 1, j]]);
            }

            if j == 0 {
                // left column, no cell left
                left = None;
            } else {
                left = Some(terrain[[i, j - 1]]);
            }

            if j == width - 1 {
                // right column, no cell right
                right = None;
            } else {
                right = Some(terrain[[i, j + 1]]);
            }

            // calculate minima
            let minima = cell < left.or(default).unwrap() &&
                cell < right.or(default).unwrap() &&
                cell < above.or(default).unwrap() &&
                cell < below.or(default).unwrap();

            if minima {
                low_point_values.push(terrain[[i, j]]);
                low_point_locations.push((i, j));
            }
        }
    }
    (low_point_locations, low_point_values)
}


/// Returns an [Array2] containing the terrain map for the problem.
fn parse_input(file_path: &str) -> Array2<u32> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let terrain_width = 100;

    let mut result = Array2::<u32>::zeros((0, terrain_width));

    let lines = contents.lines();

    for line in lines {
        let terr_x = line.
            chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();


        result.push(Axis(0), ArrayView::from(&terr_x)).unwrap();
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(566, solve_first_star());
        assert_eq!(891684, solve_second_star());
    }
}