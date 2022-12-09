use std::fs;


pub fn solve_first_star() -> usize {
    let movements = parse_input("src/rope_bridge/input.txt");

    let mut tail_path = simulate_rope_dynamics(2, movements);

    // sort and dedup tail path to get unique positions
    tail_path.sort();
    tail_path.dedup();

    tail_path.len()
}


pub fn solve_second_star() -> usize {
    let movements = parse_input("src/rope_bridge/input.txt");

    let mut tail_path = simulate_rope_dynamics(10, movements);

    // sort and dedup tail path to get unique positions
    tail_path.sort();
    tail_path.dedup();

    tail_path.len()
}


fn simulate_rope_dynamics(size: usize, movements: Vec<(char, usize)>) -> Vec<(i32, i32)> {
    // init position of rope
    let mut rope = init_rope(size);

    // vector to capture tail path as indices (x,y) for each step
    let mut tail_path = Vec::<(i32, i32)>::new();

    for (direction, steps) in movements {
        let mut steps_remaining = steps;

        while steps_remaining > 0 {
            simulate_rope_step(&mut rope, direction);

            tail_path.push(*rope.last().unwrap());

            steps_remaining -= 1;
        }
    }
    tail_path
}


fn init_rope(size: usize) -> Vec<(i32, i32)> {
    let mut rope = Vec::<(i32, i32)>::new();

    let init_x = 0_i32;
    let init_y = 0_i32;

    for _ in 0..size {
        rope.push((init_x, init_y));
    }

    rope
}

fn simulate_rope_step(rope: &mut Vec<(i32, i32)>, direction: char) {
    // update head position
    match direction {
        'L' => {
            rope[0].0 -= 1;
        }
        'D' => {
            rope[0].1 -= 1;
        }
        'U' => {
            rope[0].1 += 1;
        }
        'R' => {
            rope[0].0 += 1;
        }
        _ => { panic!("Error: Unknown direction") }
    }

    // iterate over remaining knots in rope and apply 'tail follows' dynamics
    for i in 1..rope.len() {
        rope[i] = tail_follows(rope[i - 1], rope[i]);
    }
}


fn tail_follows(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let mut new_tail_pos = tail_pos;

    let dx = head_pos.0 - tail_pos.0;
    let dy = head_pos.1 - tail_pos.1;

    match (dx.abs(), dy.abs()) {
        (0, 0) => {} // no tail movement
        (1, 0) => {} // no tail movement
        (0, 1) => {} // no tail movement
        (1, 1) => {} // no tail movement
        (2, 0) => { new_tail_pos.0 += dx.signum() } // tail movement
        (0, 2) => { new_tail_pos.1 += dy.signum() } // tail movement
        (1, 2) => {
            new_tail_pos.0 += dx.signum();
            new_tail_pos.1 += dy.signum();
        } // tail movement
        (2, 1) => {
            new_tail_pos.0 += dx.signum();
            new_tail_pos.1 += dy.signum();
        } // tail movement
        (2, 2) => {
            new_tail_pos.0 += dx.signum();
            new_tail_pos.1 += dy.signum();
        } // tail movement
        _ => { panic!("Tail movement too large") }
    }

    new_tail_pos
}


fn parse_input(file_path: &str) -> Vec<(char, usize)> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<(char, usize)>::new();

    for line in content.lines() {
        let mut line_split = line.split(' ');

        let direction = line_split.next().unwrap().chars().next().unwrap();
        let steps = line_split.next().unwrap().parse::<usize>().unwrap();

        result.push((direction, steps));
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(5981, solve_first_star());
        assert_eq!(2352, solve_second_star());
    }

    #[test]
    fn tail_movements() {
        assert_eq!((1, 0), tail_follows((2, 0), (0, 0)));
        assert_eq!((3, 2), tail_follows((4, 2), (2, 2)));
        assert_eq!((-3, -2), tail_follows((-4, -2), (-2, -2)));
        assert_eq!((-3, 2), tail_follows((-4, 2), (-2, 2)));
    }
}