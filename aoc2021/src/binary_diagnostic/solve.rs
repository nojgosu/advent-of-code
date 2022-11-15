use std::fs;

pub fn solve_first_star() -> u32 {
    let diagnostic = parse_input("src/binary_diagnostic/input.txt");

    // Calculate length of diagnostic
    let diag_length = diagnostic.len();

    let diag_entry_length = 12;

    // each entry is 12 bits in length, construct a counter
    let mut bit_counts: Vec<u32> = vec![0; diag_entry_length];

    for entry in diagnostic {
        let mut mask = 1u32;

        for b in 0..diag_entry_length {
            if (mask & entry) == mask {
                // bit set, increment count
                bit_counts[b] += 1;
            }
            // bitshift mask
            mask <<= 1;
        }
    }

    // reverse bit_counts to allow us to iterate through and
    // insert the highest to lowest bit using shift operator
    bit_counts.reverse();

    // construct gamma rate based on bit counts
    let mut gamma_rate = 0u32;
    for b in bit_counts {
        if b > (diag_length/2) as u32 {
            gamma_rate <<= 1;
            gamma_rate |= 1;
        } else {
            gamma_rate <<= 1;
        }
    }

    // construct epsilon rate by inverting relevant bits of gamma rate
    let mut epsilon_rate = !gamma_rate;
    // use mask to ensure only first 12 bits set after inverting gamma rate
    epsilon_rate &= 0x00000FFF;

    return gamma_rate * epsilon_rate;
}


pub fn solve_second_star() -> u32 {
    let diagnostic:Vec<u32> = parse_input("src/binary_diagnostic/input.txt");

    let oxygen_gen_rating = life_support_diagnostic_extractor(&diagnostic, true);

    let co2_scrub_rating = life_support_diagnostic_extractor(&diagnostic, false);

    return oxygen_gen_rating*co2_scrub_rating;
}

fn life_support_diagnostic_extractor(diagnostic: &Vec<u32>, oxygen_rating: bool) -> u32 {
    let diag_entry_length = 12;

    // set up bit mask for data extraction from diagnostic entries
    let mut mask = 0b100000000000u32;

    // initialise ones and zeros vectors of diagnostic entries
    // ones vector contains all diagnostic entries with 1's at mask bit index
    // zeros vector contains all diagnostic entries with 0's at mask bit index
    let mut ones = diagnostic.iter().collect::<Vec<_>>();
    let mut zeros = diagnostic.iter().collect::<Vec<_>>();

    for _ in 0..diag_entry_length {
        // create filters for the two bit criteria
        ones = ones.into_iter().filter(|&x| (x & mask) == mask).collect::<Vec<_>>();
        zeros = zeros.into_iter().filter(|&x| (x & mask) == 0u32).collect::<Vec<_>>();

        // determine which entries to keep based on bit criteria
        if oxygen_rating {
            // finding oxygen generator rating
            if ones.len() >= zeros.len() {
                // check if we're done
                if ones.len() == 1 {return ones[0].clone()}

                // keep the ones set of diagnostic data replacing zeros set
                zeros = ones.clone();
            } else {
                // check if we're done
                if zeros.len() == 1 {return zeros[0].clone()}

                // keep the zeros set of diagnostic data replacing ones set
                ones = zeros.clone();
            }
        } else {
            // finding CO2 scrubber rating
            if ones.len() < zeros.len() {
                // check if we're done
                if ones.len() == 1 {return ones[0].clone()}

                // keep the ones set of diagnostic data replacing zeros
                zeros = ones.clone();
            } else {
                // check if we're done
                if zeros.len() == 1 {return zeros[0].clone()}

                // keep the zeros set of diagnostic data replacng ones
                ones = zeros.clone();
            }
        }

        // shift mask
        mask >>= 1;
    }

    panic!("Unable to find rating in diagnostic")
}


fn parse_input(file_path: &str) -> Vec<u32> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let result: Vec<u32> = contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(3549854, solve_first_star());
        assert_eq!(3765399, solve_second_star());
    }
}