use std::fs;

pub fn solve_first_star() -> u32 {
    let mut population = parse_input("src/lanternfish/input.txt");

    simulate_population(&mut population, 80);

    population.len() as u32
}


pub fn solve_second_star() -> u64 {
    let population = parse_input("src/lanternfish/input.txt");

    let result = simulate_population_optimised(population, 256);

    result.population_count()
}

fn simulate_population(population: &mut Vec<Lanternfish>, generations: u32) {
    for i in 0..generations {
        let mut spawn = Vec::<Lanternfish>::new();

        population
            .iter_mut()
            .for_each(|fish| if let Some(fish_spawn) = fish.breed() { spawn.push(fish_spawn) });

        population.append(&mut spawn);
    }
}

//TODO: Optimise
fn simulate_population_optimised(initial_population: Vec<Lanternfish>, generations: u32) -> LanternfishPopulation {
    let mut population = LanternfishPopulation::new(initial_population);

    for i in 0..generations {
        population.breed();
    }

    population
}


fn parse_input(file_path: &str) -> Vec<Lanternfish> {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut population = Vec::<Lanternfish>::new();

    contents
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .for_each(|x| population.push(Lanternfish::new(x)));

    population
}

struct Lanternfish {
    breeding_cycle: u32,
}

impl Lanternfish {
    fn breed(&mut self) -> Option<Lanternfish> {
        if self.breeding_cycle == 0 {
            // spawn new fish and restart breeding cycle
            self.breeding_cycle = 6;
            return Some(Lanternfish::new(8));
        } else {
            self.breeding_cycle -= 1u32;
        }

        None
    }

    fn new(breeding_cycle: u32) -> Lanternfish {
        Lanternfish {
            breeding_cycle,
        }
    }
}

struct LanternfishPopulation {
    cohorts: Vec<(u64, u64)>,
}

impl LanternfishPopulation {
    fn breed(&mut self) {
        let mut new_spawn = 0u64;
        let mut just_given_birth = 0u64;

        for (cycle, num_fish) in self.cohorts.iter_mut() {
            match cycle {
                // breed more fish
                0 => {
                    new_spawn = *num_fish;
                    just_given_birth = *num_fish;
                    *num_fish = 0; // kill this cohort as it'll be consolidated with the other 6's
                }
                // reduce breeding cycle count for other cohorts
                _ => { *cycle -= 1 }
            };
        }


        if let Some((_, num_fish)) = self.cohorts.iter_mut().find(|(cycle, _)| *cycle == 6) {
            // add fish who just gave birth to cohort with breeding cycle of 6
            *num_fish += just_given_birth;
        } else {
            // no cohort with breeding cycle of 6. Add one
            let new_cohort = (6u64, just_given_birth);

            self.cohorts.push(new_cohort);
        }

        // add new spawn to the population
        self.cohorts.push((8, new_spawn));

        // remove any empty cohorts
        self.cohorts.retain(|(_, num_fish)| { *num_fish != 0 });
    }

    fn population_count(&self) -> u64 {
        let mut count = 0u64;
        self.cohorts.iter().for_each(|(_, num_fish)| {
            count += num_fish;
        });
        count
    }

    fn new(fish: Vec<Lanternfish>) -> LanternfishPopulation {
        let mut initial_pop = Vec::<(u64, u64)>::new();

        for cycle in 0..9 {
            let num_fish = fish.iter().filter(|x| x.breeding_cycle == cycle).count();
            initial_pop.push((cycle as u64, num_fish as u64));
        }

        LanternfishPopulation {
            cohorts: initial_pop,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(345387, solve_first_star());
        assert_eq!(1574445493136, solve_second_star());
    }
}