use std::cmp::Ordering;
use std::fs;
use itertools::Itertools;
use serde::Deserialize;
use crate::distress_signal::solve::Packet::{Number};


pub fn solve_first_star() -> usize {
    let packet_pairs = parse_input_pairs("src/distress_signal/input.txt");

    let mut sum = 0_usize;

    for (index, (left, right)) in packet_pairs.iter().enumerate() {
        if left < right { sum += index + 1 }
    }

    sum
}


pub fn solve_second_star() -> usize {
    let mut packets = parse_input("src/distress_signal/input.txt");

    // divider packets
    let div_2 = serde_json::from_str::<Packet>("[[2]]").unwrap();
    let div_6 = serde_json::from_str::<Packet>("[[6]]").unwrap();

    // add divider packets
    packets.push(div_2.clone());
    packets.push(div_6.clone());

    // sort packets
    packets.sort();

    // find index of divider packets
    let (i, _) = packets.iter().find_position(|&x| *x == div_2).unwrap();
    let (j, _) = packets.iter().find_position(|&x| *x == div_6).unwrap();

    // index starts at 1 for puzzle, not 0
    (i+1)*(j+1)
}


#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(untagged)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}


impl Packet {
    fn unwrap_num(self) -> u32 {
        match self {
            Number(a) => { a }
            Packet::List(_) => { panic!("Cannot unwrap_num Packet. Packet is a List") }
        }
    }

    fn unwrap_vec(self) -> Vec<Packet> {
        match self {
            Packet::List(a) => { a }
            Number(_) => { panic!("Cannot unwrap_vec Packet. Packet is a Number") }
        }
    }

    fn as_list(&self) -> Packet {
        match self {
            Number(a) => { Packet::List(vec![Number(*a)]) }
            _ => { self.clone() }
        }
    }
}


impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Number(a), Number(b)) => { Some(a.cmp(b)) }
            (l, r) => {
                let x = l.as_list().unwrap_vec();
                let y = r.as_list().unwrap_vec();

                // recursively search for first ordering that isn't 'Equal'
                // if none are found, return comparison of lengths of list
                Some(x.iter().zip(y.iter())
                    .map(|(a, b)| a.partial_cmp(b).unwrap())
                    .find(|&x| x != Ordering::Equal)
                    .unwrap_or_else(|| x.len().cmp(&y.len())))
            }
            _ => { None }
        }
    }
}


impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn parse_input_pairs(file_path: &str) -> Vec<(Packet, Packet)> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<(Packet, Packet)>::new();

    // parse pairs
    let mut packets = content.lines();

    loop {
        let l: Packet;
        let r: Packet;
        if let Some(packet_str) = packets.next() {
            l = serde_json::from_str::<Packet>(packet_str).unwrap();
        } else { break; }

        if let Some(packet_str) = packets.next() {
            r = serde_json::from_str::<Packet>(packet_str).unwrap();
        } else { break; }

        // consume empty line
        packets.next();

        result.push((l, r));
    }

    result
}


fn parse_input(file_path: &str) -> Vec<(Packet)> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    let mut result = Vec::<(Packet)>::new();

    // parse pairs
    let packets = content.lines();

    for packet_str in packets {
        if let Ok(packet) = serde_json::from_str::<Packet>(packet_str) {
            result.push(packet);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(5717, solve_first_star());
        assert_eq!(25935, solve_second_star());
    }
}