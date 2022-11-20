use std::cmp::Ordering;
use std::fs;
use ndarray::{Array2};


pub fn solve_first_star() -> u32 {
    let lines = parse_input("src/hydrothermal_venture/input.txt");

    let grid_x_dimension = 1000;
    let grid_y_dimension = 1000;

    let mut grid = Array2::<u32>::zeros((grid_x_dimension, grid_y_dimension));

    let straight_lines = lines.iter().filter(|line| {
            line.p1.x == line.p2.x || line.p1.y == line.p2.y
    });

    // Map lines
    for line in straight_lines {
        line.plot_line(&mut grid);
    }

    let dangerous_grid_points = grid.map(|x| {
        u32::from(*x >= 2u32)
    });

    dangerous_grid_points.sum()

}


pub fn solve_second_star() -> u32 {
    let lines = parse_input("src/hydrothermal_venture/input.txt");

    let grid_x_dimension = 1000;
    let grid_y_dimension = 1000;

    let mut grid = Array2::<u32>::zeros((grid_x_dimension, grid_y_dimension));

    // Map lines
    for line in lines {
        line.plot_line(&mut grid);
    }


    let dangerous_grid_points = grid.map(|x| {
        u32::from(*x >= 2u32)
    });

    dangerous_grid_points.sum()
}

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    /// Plots the line on the grid provided.
    ///
    /// Assumes diagonal lines are at 45 degrees, such that x and y increment evenly.
    fn plot_line(&self, grid:&mut Array2<u32>) {

        let mut cursor = (self.p1.x, self.p1.y);

        loop {
            // increment grid
            grid[[cursor.0 as usize, cursor.1 as usize]] += 1;

            // check if we're done
            if cursor.0 == self.p2.x && cursor.1 == self.p2.y {break;}

            // move cursor in direction of final point
            let x_increment = match cursor.0.cmp(&self.p2.x) {
                Ordering::Greater => -1,
                Ordering::Equal => 0,
                Ordering::Less => 1,
            };

            let y_increment = match cursor.1.cmp(&self.p2.y) {
                Ordering::Greater => -1,
                Ordering::Equal => 0,
                Ordering::Less => 1,
            };


            cursor.0 = (cursor.0 as i32 + x_increment) as u32;
            cursor.1 = (cursor.1 as i32 + y_increment) as u32;
        }

    }
}


fn parse_input(file_path: &str) -> Vec<Line> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let entries = contents
        .lines();

    let mut result = Vec::<Line>::new();

    for entry in entries {
        let mut temp = Vec::<Point>::new();

        for point in entry.split("->") {
            let data = point
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let point = Point {
                x: *data.first().unwrap(),
                y: *data.last().unwrap(),
            };

            temp.push(point);
        }

        let line = Line {
            p1: temp.first().unwrap().clone(),
            p2: temp.last().unwrap().clone(),
        };

        result.push(line);
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(6710, solve_first_star());
        assert_eq!(20121, solve_second_star());
    }
}