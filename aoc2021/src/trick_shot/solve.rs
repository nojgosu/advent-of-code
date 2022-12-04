use std::sync::mpsc::channel;
use itertools::{Itertools, max};
use rayon::prelude::*;


pub fn solve_first_star() -> i32 {
    let initial_velocities = generate_initial_velocities();

    // Set up channel to receive results from parallel iterator
    let (sender, receiver) = channel();

    // Run simulations in parallel using Rayon
    initial_velocities.par_iter().for_each_with(sender, |s, (x_dot_init, y_dot_init)| {
        let (max_y, status) = run_simulation(x_dot_init, y_dot_init);

        // send result if objective met
        match status {
            ObjectiveStatus::Incomplete => {}
            ObjectiveStatus::Impossible => {}
            ObjectiveStatus::Complete => { s.send((x_dot_init, y_dot_init, max_y)).unwrap(); }
        }
    });

    // Collect solution results
    let solutions: Vec<_> = receiver.into_iter().collect();

    // find max_y in solution results
    let solution = solutions
        .iter()
        .max_by(|(_, _, a), (_, _, b)| a.cmp(b));

    let (x_init, y_init, y_max) = solution.unwrap();

    *y_max
}


pub fn solve_second_star() -> usize {
    let initial_velocities = generate_initial_velocities();

    // Set up channel to receive results from parallel iterator
    let (sender, receiver) = channel();

    // Run simulations in parallel using Rayon
    initial_velocities.par_iter().for_each_with(sender, |s, (x_dot_init, y_dot_init)| {
        let (max_y, status) = run_simulation(x_dot_init, y_dot_init);

        // send result if objective met
        match status {
            ObjectiveStatus::Incomplete => {}
            ObjectiveStatus::Impossible => {}
            ObjectiveStatus::Complete => { s.send((x_dot_init, y_dot_init, max_y)).unwrap(); }
        }
    });

    // Collect solution results
    let solutions: Vec<_> = receiver.into_iter().collect();

    // Return number of solution
    solutions.len()
}


#[derive(PartialEq, Debug)]
enum ObjectiveStatus {
    Impossible,
    Incomplete,
    Complete,
}


fn generate_initial_velocities() -> Vec<(i32, i32)> {
    let mut initial_velocities = Vec::<(i32, i32)>::new();

    for x_dot_init in 0..200 {
        for y_dot_init in -200..500 {
            initial_velocities.push((x_dot_init, y_dot_init));
        }
    }

    initial_velocities
}


fn run_simulation(x_dot_init: &i32, y_dot_init: &i32) -> (i32, ObjectiveStatus) {
    // initialise simulation
    let mut x_dot = *x_dot_init;
    let mut y_dot = *y_dot_init;
    let mut x = 0i32; // initial x position
    let mut y = 0i32; // initial y position

    // track max y position
    let mut max_y = 0i32;

    let mut status = ObjectiveStatus::Incomplete;

    while check_objective(x, y, x_dot, y_dot) == ObjectiveStatus::Incomplete {
        (x, y, x_dot, y_dot) = particle_simulation_step(x, y, x_dot, y_dot);

        max_y = max_y.max(y);

        status = check_objective(x, y, x_dot, y_dot);
    }
    (max_y, status)
}


/// performs a step in the particle simulation
fn particle_simulation_step(x: i32, y: i32, x_dot: i32, y_dot: i32) -> (i32, i32, i32, i32) {
    let mut next_x = x;
    let mut next_y = y;

    // update position
    next_x += x_dot;
    next_y += y_dot;

    let mut next_x_dot = x_dot;
    let mut next_y_dot = y_dot;

    // update velocity
    if x_dot == 0 {
        next_x_dot = 0;
    } else if next_x_dot.is_negative() {
        next_x_dot += 1;
    } else {
        next_x_dot -= 1;
    }

    next_y_dot -= 1;

    (next_x, next_y, next_x_dot, next_y_dot)
}


fn check_objective(x: i32, y: i32, x_dot: i32, y_dot: i32) -> ObjectiveStatus {
    let x_objective = 150..193 + 1;
    let y_objective = -136..-86 + 1;

    if x_objective.contains(&x) && y_objective.contains(&y) {
        return ObjectiveStatus::Complete;
    }

    let y_min = y_objective.min().unwrap();
    let x_min = x_objective.clone().min().unwrap();
    let x_max = x_objective.clone().max().unwrap();

    // check if objective is impossible - this is incorrect
    if x_dot.is_negative() && x < x_min { return ObjectiveStatus::Impossible; }
    if x_dot.is_positive() && x > x_max { return ObjectiveStatus::Impossible; }
    if x_dot == 0 && !x_objective.contains(&x) { return ObjectiveStatus::Impossible; }
    if y_dot.is_negative() && y < y_min { return ObjectiveStatus::Impossible; }


    ObjectiveStatus::Incomplete
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(9180, solve_first_star());
        assert_eq!(3767, solve_second_star());
    }

    #[test]
    fn objective_test() {
        assert_eq!(ObjectiveStatus::Incomplete, check_objective(0, 0, 24, 135));
        assert_eq!(ObjectiveStatus::Impossible, check_objective(140, 200, 0, 135));
        assert_eq!(ObjectiveStatus::Incomplete, check_objective(140, 200, 1, 135));
        assert_eq!(ObjectiveStatus::Complete, check_objective(157, -132, 1, 135));
    }
}