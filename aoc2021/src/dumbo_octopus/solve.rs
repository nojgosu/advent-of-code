use std::fs;
use ndarray::{Array2, ArrayView, Axis};


const TRIGGER_ENERGY: u32 = 10_u32;
const TICKS: u32 = 100_u32;

pub fn solve_first_star() -> u64 {
    let mut octopii = parse_input("src/dumbo_octopus/input.txt");

    let mut flash_count = 0u64;

    for _ in 0..TICKS {
        // initialise map of flashed octopii for tick cycle
        let mut flash_map = octopii.map(|_| false);

        // increase energy level due to tick
        octopii.map_mut(|o| *o += 1);

        // process octopii flashing
        while flash_octopii(&mut octopii, &mut flash_map, &mut flash_count) {}

        // de-energise flashed octopii
        deenergise_octopii(&mut octopii);
    }

    flash_count
}

pub fn solve_second_star() -> u64 {
    let mut octopii = parse_input("src/dumbo_octopus/input.txt");

    let mut sync_tick = 0u64;
    let mut synchronised = false;

    let mut flash_count = 0u64;

    while !synchronised {
        // initialise map of flashed octopii for tick cycle
        let mut flash_map = octopii.map(|_| false);

        // increase energy level due to tick
        octopii.map_mut(|o| *o += 1);

        // process octopii flashing
        while flash_octopii(&mut octopii, &mut flash_map, &mut flash_count) {}

        // de-energise flashed octopii
        deenergise_octopii(&mut octopii);

        // check for end condition, all octopii have flashed
        if flash_map.iter().all(|o| *o == true) {
            synchronised = true;
        }

        sync_tick += 1;
    }

    sync_tick
}


/// Processes the octopii grid and 'flashes' any octopus with sufficient energy which haven't
/// flashed this tick
fn flash_octopii(octopii: &mut Array2<u32>, flash_map: &mut Array2<bool>, flash_count: &mut u64) -> bool {
    // action any flashes and update flash_map, if an octopus flashed, return true, otherwise false
    let mut flashed = false;

    let (height, width) = octopii.dim();

    //println!("height = {:?}\twidth = {:?}", height, width);

    // maintain flash impacts to update later
    let mut flash_energy = octopii.map(|_| 0u32);

    for ((row, col), o) in octopii.indexed_iter() {

        //println!("row = {:?}\tcol = {:?}", row, col);
        if *o >= TRIGGER_ENERGY {
            // check if it's already flashed this tick
            if !flash_map[[row, col]] {
                flashed = true;

                // flash octopus
                *flash_count += 1;
                flash_map[[row, col]] = true;

                // update cumulated flash_energy for neighbours
                flash_energy[[row, col]] += 1;

                // construct indices as int so we can filter out negatives
                let indices: Vec<(i32, i32)> = vec![
                    (row as i32 - 1, col as i32 - 1),
                    (row as i32 - 1, col as i32),
                    (row as i32 - 1, col as i32 + 1),
                    (row as i32, col as i32 - 1),
                    (row as i32, col as i32),
                    (row as i32, col as i32 + 1),
                    (row as i32 + 1, col as i32 - 1),
                    (row as i32 + 1, col as i32),
                    (row as i32 + 1, col as i32 + 1),
                ];

                // filter out invalid indices
                let neighbours = indices
                    .iter()
                    .filter(|&(r, c)|
                        *r >= 0 && *c >= 0 && *r < height as i32 && *c < width as i32)
                    .collect::<Vec<_>>();

                for (r, c) in neighbours {
                    flash_energy[[*r as usize, *c as usize]] += 1;
                }
            }
        }
    }

    // add flash energy to octopii
    octopii.zip_mut_with(&flash_energy, |o, e| { *o += *e });

    flashed
}

/// De-energise any octopii with an energy level over the flash point
fn deenergise_octopii(octopii: &mut Array2<u32>) {
    octopii.iter_mut().for_each(|x| {
        if *x >= TRIGGER_ENERGY {
            *x = 0;
        }
    });
}


fn parse_input(file_path: &str) -> Array2<u32> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    // get dimensions of octopii grid
    let width = contents.lines().next().unwrap().len();

    let mut result = Array2::<u32>::zeros((0, width));

    let lines = contents.lines();

    for line in lines {
        let octopii = line.
            chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        result.push(Axis(0), ArrayView::from(&octopii)).unwrap();
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(1652, solve_first_star());
        assert_eq!(220, solve_second_star());
    }
}