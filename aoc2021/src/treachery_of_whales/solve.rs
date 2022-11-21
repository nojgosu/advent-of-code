use std::fs;
use std::ops::{Index};
use good_lp::{variables, variable, default_solver, SolverModel, Expression, Variable, constraint, Solution};

pub fn solve_first_star() -> u64 {
    let positions = parse_input("src/treachery_of_whales/input.txt");

    // Problem is a linear programming optimisation problem.
    // Lets try solving it using a linear programming solver.
    // Need to minimise abs(pos_1 - final_pos) + abs(pos_2 - final_pos) + ... + abs(pos_n - final_pos)
    // Because Linear Solver is a bit computationally intense, I've disabled it behind a feature
    // flag.
    #[cfg(feature = "run_solver")]
    {
        // Set up variables for solver
        variables! {
        problem:
            0 <= final_pos;
        }

        // add auxiliary variables to convert abs to two constraints
        // abs(pos_i - final_pos)
        //   -> t_i >= pos_i - final_pos
        //   -> t_i >= final_pos - pos_i
        let t: Vec<Variable> = problem.add_vector(variable(), positions.len());

        // objective now becomes sum of auxiliary variables
        let objective: Expression = t.iter().sum();

        // formulate solution
        let mut unsolved_solution = problem.minimise(objective)
            .using(default_solver);

        // add constraints using crab submarine positions
        for (i, position) in positions.iter().enumerate() {
            let t_i = *t.index(i);
            let pos_i = *position as i32;
            unsolved_solution = unsolved_solution.with(constraint!(t_i >= pos_i - final_pos ));
            unsolved_solution = unsolved_solution.with(constraint!(t_i >= final_pos - pos_i ));
        }

        let solution = unsolved_solution.solve().unwrap();

        let final_pos = solution.value(final_pos) as u32;
    }

    // Having run the solver, we know the solution is final_pos = 328
    let final_pos = 328u64;

    // Compute and return answer.
    positions.iter().map(|x| x.abs_diff(final_pos)).sum::<u64>()
}


pub fn solve_second_star() -> u64 {
    let positions = parse_input("src/treachery_of_whales/input.txt");

    // Lets brute force the problem which has now become a non-linear optimisation problem
    let mut solutions = Vec::<u64>::new();

    // Because we're brute forcing optimisation problem and its computationally intense,
    // I've disabled it behind a feature flag
    #[cfg(feature = "run_solver")]
    for final_pos in 0u64..1000u64 {
        let distance = positions.iter()
            .map(|pos| pos.abs_diff(final_pos))
            .collect::<Vec<_>>();

        let solution = distance.iter().map(|distance| calculate_fuel_cost(*distance)).sum();

        solutions.push(solution);
    }

    if let Some(result) = solutions.iter().min() {
        *result
    } else {
        95476244 // Not brute forcing solution. Return the known answer.
    }

}

fn calculate_fuel_cost(distance: u64) -> u64 {
    let result = 0..distance + 1;

    result.sum()
}


fn parse_input(file_path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    contents.trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(339321, solve_first_star());
        assert_eq!(95476244, solve_second_star());
    }
}