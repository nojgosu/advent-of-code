use std::collections::{BTreeMap, HashMap};
use std::fs;


pub fn solve_first_star() -> u64 {
    let (template, rules) = parse_input("src/extended_polymerization/input.txt");

    let mut polymer = template;

    for _ in 0..10 {
        polymerise(&mut polymer, &rules);
    }

    // find most common and least common elements and count
    let mut counts = BTreeMap::new();

    for c in polymer.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let (_, max) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    let (_, min) = counts.iter().min_by_key(|&(_, count)| count).unwrap();

    max - min
}

pub fn solve_second_star() -> u64 {
    let (template, rules) = parse_input("src/extended_polymerization/input.txt");

    // Due to exponential growth, method for solve_first_star() doesn't scale.
    // Instead manage the polymer not as a string, but as a hashmap of element pairs and counts.

    // initialise hashmap
    let mut polymers = init_polymer_hashmap(template);

    // Polymerise for 40 steps
    for _ in 0..40 {
        polymers = hash_polymerise(polymers, &rules);
    }


    // Count occurrences of elements in polymer
    let mut counts = BTreeMap::new();

    for (polymer, count) in polymers {
        // count both chars
        let mut chars = polymer.chars();

        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();

        counts.entry(c1).and_modify(|x| *x += count).or_insert(count);
        counts.entry(c2).and_modify(|x| *x += count).or_insert(count);
    }

    // Because of the way we're storing the polymer, we've been double counting elements.
    // We need to half the element counts and take the ceiling to account for instances of
    // odd number of elements
    for (_, count) in counts.iter_mut() {
        *count = (*count as f64 / 2_f64).ceil() as u64
    }

    let (_, max) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    let (_, min) = counts.iter().min_by_key(|&(_, count)| count).unwrap();

    max - min
}


fn polymerise(polymer: &mut String, ruleset: &HashMap<String, char>) {
    let template = polymer.clone();

    let mut inserted_elems = 0;

    for i in 0..template.len() - 1 {
        let polymer_pair = &template[i..i + 2];

        // look up polymer based on rule
        if let Some(new_elem) = ruleset.get(polymer_pair) {
            // insert new element
            polymer.insert(i + inserted_elems + 1, *new_elem);

            // keep track of inserted element counts to manage offset to index in polymer
            inserted_elems += 1;
        }
    }
}


fn init_polymer_hashmap(template: String) -> HashMap<String, u64> {
    let mut polymer_chain = HashMap::<String, u64>::new();

    for i in 0..template.len() - 1 {
        let polymer_pair = &template[i..i + 2];

        polymer_chain
            .entry(polymer_pair.to_string())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    polymer_chain
}


fn hash_polymerise(polymer_chain: HashMap<String, u64>, ruleset: &HashMap<String, char>) -> HashMap<String, u64> {
    let mut new_polymer_chain = HashMap::<String, u64>::new();

    // breakdown existing polymer_chain and build a new one
    for (polymer, count) in polymer_chain.into_iter() {
        let poly_string = polymer;

        if let Some(&new_elem) = ruleset.get(poly_string.as_str()) {
            let mut polymer_chars = poly_string.chars();

            let mut poly_1 = String::from(polymer_chars.next().unwrap());
            poly_1.push(new_elem);

            let mut poly_2 = String::from(polymer_chars.next().unwrap());
            poly_2.insert(0, new_elem);

            // add new polymers
            new_polymer_chain.entry(poly_1).and_modify(|x| *x += count).or_insert(count);
            new_polymer_chain.entry(poly_2).and_modify(|x| *x += count).or_insert(count);
        }
    }

    new_polymer_chain
}


fn parse_input(file_path: &str) -> (String, HashMap<String, char>) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    let mut lines = contents.lines();

    // parse template
    let template = lines.next().unwrap().to_string();

    let mut polymerisation_ruleset = HashMap::<String, char>::new();

    // drop empty line
    lines.next();

    // parse ruleset
    for line in lines {
        let polymer_rule = line.split("->").collect::<Vec<_>>();

        polymerisation_ruleset.insert(polymer_rule[0].trim().to_string(),
                                      polymer_rule[1].trim().chars().next().unwrap());
    }

    (template, polymerisation_ruleset)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(2967, solve_first_star());
        assert_eq!(3692219987038, solve_second_star());
    }

    #[test]
    fn test_poylmerise_hash() {
        let (template, rules) = parse_input("src/extended_polymerization/test_input.txt");

        let mut polymer = init_polymer_hashmap(template);

        let polymer = hash_polymerise(polymer, &rules);
        assert_eq!(init_polymer_hashmap("NCNBCHB".to_string()), polymer);

        let polymer = hash_polymerise(polymer, &rules);
        assert_eq!(init_polymer_hashmap("NBCCNBBBCBHCB".to_string()), polymer);

        let polymer = hash_polymerise(polymer, &rules);
        assert_eq!(init_polymer_hashmap("NBBBCNCCNBBNBNBBCHBHHBCHB".to_string()), polymer);

        let polymer = hash_polymerise(polymer, &rules);
        assert_eq!(init_polymer_hashmap("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string()), polymer);
    }

    #[test]
    fn test_poylmerise() {
        let (template, rules) = parse_input("src/extended_polymerization/test_input.txt");

        let mut polymer = template;

        polymerise(&mut polymer, &rules);
        assert_eq!("NCNBCHB".to_string(), polymer);

        polymerise(&mut polymer, &rules);
        assert_eq!("NBCCNBBBCBHCB".to_string(), polymer);

        polymerise(&mut polymer, &rules);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB".to_string(), polymer);

        polymerise(&mut polymer, &rules);
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string(), polymer);
    }
}