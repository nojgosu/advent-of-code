use std::fs;
use std::fmt::Write;
use std::ops::Add;


pub fn solve_first_star() -> u64 {
    let initial_packet = parse_input("src/packet_decoder/input.txt");

    let mut packet = &initial_packet[..];
    let mut version_accumulator = 0u64;

    while !packet.is_empty() {
        let (version, _, remaining_packet) = process_packet(packet);

        version_accumulator += version;
        packet = remaining_packet;
    }

    version_accumulator
}


pub fn solve_second_star() -> u64 {
    let initial_packet = parse_input("src/packet_decoder/input.txt");

    let mut packet = &initial_packet[..];
    let mut result_accumulator = 0u64;

    while !packet.is_empty() {
        let (_, result, remaining_packet) = process_packet(packet);

        result_accumulator += result;
        packet = remaining_packet;
    }

    result_accumulator
}


/// Processes the header of the packet and decodes the version and type ID, returning the
/// remaining slice of the packet for further processing
fn process_header(packet: &str) -> (u64, u64, &str) {
    let version = u64::from_str_radix(&packet[0..3], 2).unwrap();

    let packet_type_id = u64::from_str_radix(&packet[3..6], 2).unwrap();

    let data_payload = &packet[6..];

    (version, packet_type_id, data_payload)
}


/// Processes the data section of a literal packet, returning the decimal number stored whilst
/// returning the remaining slice of the packet for further processing
fn process_literal_data(packet: &str) -> (u64, &str) {
    let mut more_data = true;

    let mut literal = String::new();

    let mut window = packet;

    while more_data {
        if window.starts_with('1') {
            more_data = true;
        } else {
            more_data = false;
        }

        // add bits to literal
        literal = literal.add(&window[1..5]);

        // update window
        window = &window[5..];
    }

    // convert bits to unsigned int
    (u64::from_str_radix(&literal, 2).unwrap(), window)
}


/// Processes the data section of an operator packet.
/// Returns the cumulative value of all sub packet versions, the result
/// of operator on the sub packet, and the remaining slice of the packet for further processing
fn process_operator_data(packet: &str, packet_type: u64) -> (u64, u64, &str) {
    let mut version_accumulator = 0u64;
    let mut results = Vec::<u64>::new();

    // calculate packet size length based on length id type
    if packet.starts_with('0') {
        // length_id_type = 0
        // fixed packet slice containing n sub packets
        let length = 15;

        // calculate size of sub packets
        let sub_packets_size = usize::from_str_radix(&packet[1..length + 1], 2).unwrap();

        // calculate start and end of sub packet
        let start = length + 1;
        let end = length + 1 + sub_packets_size;

        let mut sub_packets = &packet[start..end];

        // process sub packets
        while !sub_packets.is_empty() {
            let (version, result, remaining_packet) = process_packet(sub_packets);
            version_accumulator += version;

            results.push(result);

            sub_packets = remaining_packet;
        }

        let result = perform_operator(results, packet_type);

        (version_accumulator, result, &packet[end..])
    } else {
        // length_id_type = 1
        // number of sub packets
        let length = 11;

        // calculate number of sub packets
        let sub_packets_num = usize::from_str_radix(&packet[1..length + 1], 2).unwrap();

        // calculate start and end of sub packet (don't know end)
        let start = length + 1;

        let mut remaining_packet = &packet[start..];

        // process sub packets
        for _ in 0..sub_packets_num {
            let (version, result, next_packet) = process_packet(remaining_packet);

            remaining_packet = next_packet;

            version_accumulator += version;

            results.push(result);
        }

        let result = perform_operator(results, packet_type);

        (version_accumulator, result, remaining_packet)
    }
}


/// Performs the operation determined by packet_type id on the sub packet results vector
/// and returns the result
fn perform_operator(results: Vec<u64>, packet_type: u64) -> u64 {
    match packet_type {
        0 => {results.iter().sum()},
        1 => {results.iter().product()},
        2 => {*results.iter().min().unwrap()},
        3 => {*results.iter().max().unwrap()},
        5 => {results.first().unwrap().gt(results.last().unwrap()) as u64},
        6 => {results.first().unwrap().lt(results.last().unwrap()) as u64},
        7 => {results.first().unwrap().eq(results.last().unwrap()) as u64},
        _ => {panic!("Error: Unknown packet type")},
    }
}


/// Recursive function that processes packets and accumulates their cumulative packet version numbers
/// and the overall result of the packet. Also returns the next packet for processing, if further
/// processing required.
fn process_packet(packet: &str) -> (u64, u64, &str) {
    let mut version_accumulator = 0u64;
    let mut result_accumulator = 0u64;
    let mut next_packet = "";

    if packet.len() < 11 {
        // not enough packet left to process. Return empty string.
        return (result_accumulator, version_accumulator, next_packet);
    }

    let (version, packet_type, data) = process_header(packet);

    version_accumulator += version;

    // process packet data
    if packet_type == 4 {
        // literal packet
        let (literal, remaining_packet) = process_literal_data(data);

        next_packet = remaining_packet;

        result_accumulator += literal;
    } else {
        // operator packet
        let (version, result, remaining_packet) = process_operator_data(data, packet_type);

        version_accumulator += version;
        result_accumulator += result;
        next_packet = remaining_packet;
    }

    (version_accumulator, result_accumulator, next_packet)
}


/// Converts the hex to binary string representation for packet parsing
fn parse_hex_to_binary(contents: &str) -> String {
    let mut hex_bytes = Vec::<u64>::new();

    for c in contents.chars() {
        if let Some(d) = c.to_digit(16) {
            hex_bytes.push(d as u64);
        }
    }

    let mut result = String::new();

    for byte in hex_bytes {
        write!(&mut result, "{:04b}", byte).expect("bytes should be properly parsed earlier");
    }
    result
}


fn parse_input(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    parse_hex_to_binary(&contents)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(895, solve_first_star());
        assert_eq!(1148595959144, solve_second_star());
    }

    #[test]
    fn binary_parser() {
        let packet = parse_hex_to_binary("D2FE28");
        assert_eq!("110100101111111000101000", packet);

        let packet = parse_hex_to_binary("38006F45291200");
        assert_eq!("00111000000000000110111101000101001010010001001000000000", packet);

        let packet = parse_hex_to_binary("EE00D40C823060");
        assert_eq!("11101110000000001101010000001100100000100011000001100000", packet);
    }

    #[test]
    fn processing_header() {
        let packet = parse_hex_to_binary("D2FE28");
        let (version, packet_type_id, data) = process_header(&packet);
        assert_eq!(6, version);
        assert_eq!(4, packet_type_id);
        assert_eq!("101111111000101000", data);

        let packet = parse_hex_to_binary("38006F45291200");
        let (version, packet_type_id, data) = process_header(&packet);
        assert_eq!(1, version);
        assert_eq!(6, packet_type_id);
        assert_eq!("00000000000110111101000101001010010001001000000000", data);

        let packet = parse_hex_to_binary("EE00D40C823060");
        let (version, packet_type_id, data) = process_header(&packet);
        assert_eq!(7, version);
        assert_eq!(3, packet_type_id);
        assert_eq!("10000000001101010000001100100000100011000001100000", data);
    }

    #[test]
    fn processing_literal_packet() {
        let packet = parse_hex_to_binary("D2FE28");
        let (_, _, data) = process_header(&packet);
        let (num, next_packet) = process_literal_data(data);
        assert_eq!("000", next_packet);
        assert_eq!(2021, num);
    }

    #[test]
    fn processing_operator_packet() {
        let packet = parse_hex_to_binary("38006F45291200");
        let (header_version, packet_type, data) = process_header(&packet);
        let (version, result, next_packet) = process_operator_data(data, packet_type);
        assert_eq!("0000000", next_packet);
        assert_eq!(1, result);
        assert_eq!(9, version + header_version);
    }

    #[test]
    fn processing_packet() {
        let packet = parse_hex_to_binary("EE00D40C823060");
        let (version, result, next_packet) = process_packet(&packet);
        assert_eq!("00000", next_packet);
        assert_eq!(3, result);
        assert_eq!(14, version);
    }
}