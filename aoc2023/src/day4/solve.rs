use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::multi::{many0, many_till};
use nom::sequence::terminated;
use nom::IResult;
use std::collections::{HashSet};
use std::fs;

pub fn solve_first_star() -> usize {
    let cards = parse_input("src/day4/input.txt");

    cards.iter().fold(0_usize, |acc, x| acc + x.value())
}

pub fn solve_second_star() -> usize {
    let cards = parse_input("src/day4/input.txt");

    count_winning_cards(cards)
}

#[derive(PartialEq, Debug)]
struct ScratchCard {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    copies: usize,
}

impl ScratchCard {
    fn value(&self) -> usize {
        let winners = self
            .winning_numbers
            .intersection(&self.numbers)
            .collect::<Vec<&usize>>();

        if winners.is_empty() {
            return 0;
        };

        2_usize.pow(winners.len() as u32 - 1)
    }

    fn winning_count(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

fn count_winning_cards(mut cards: Vec<ScratchCard>) -> usize {
    for i in 0..cards.len() {
        let copies = cards[i].copies;
        let winners = cards[i].winning_count();

        cards[i + 1..]
            .iter_mut()
            .take(winners)
            .for_each(|x| x.copies += copies * 1);
    }

    return cards.iter().fold(0_usize, |acc, x| acc + x.copies);
}

fn parse_input(file_path: &str) -> Vec<ScratchCard> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    content
        .lines()
        .map(|line| {
            let (_, card) = parse_card(line).unwrap();
            card
        })
        .collect()
}

fn parse_card(input: &str) -> IResult<&str, ScratchCard> {
    let (rest, _) = space0(input)?;
    let (rest, _) = tag("Card")(rest)?;
    let (rest, _) = space0(rest)?;
    let (rest, id) = terminated(digit1, tag(":"))(rest)?;
    let (rest, (winning_numbers, _)) = many_till(parse_number, tag("|"))(rest)?;
    let (rest, numbers) = many0(parse_number)(rest)?;

    Ok((
        rest,
        ScratchCard {
            id: id.parse().unwrap(),
            winning_numbers: winning_numbers
                .iter()
                .map(|&x| x.parse::<usize>().unwrap())
                .collect(),
            numbers: numbers
                .iter()
                .map(|&x| x.parse::<usize>().unwrap())
                .collect(),
            copies: 1,
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, &str> {
    let (rest, _) = space0(input)?;
    let (rest, num) = digit1(rest)?;
    let (rest, _) = space0(rest)?;

    Ok((rest, num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(23673, solve_first_star());
        assert_eq!(12263631, solve_second_star());
    }

    #[test]
    fn test_first_star_solution() {
        let cards = parse_input("src/day4/test_input.txt");

        let result = cards.iter().fold(0_usize, |acc, x| acc + x.value());

        assert_eq!(13, result);
    }

    #[test]
    fn test_second_star_solution() {
        let cards = parse_input("src/day4/test_input.txt");

        assert_eq!(30, count_winning_cards(cards));
    }
}
