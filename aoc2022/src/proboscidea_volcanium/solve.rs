use std::collections::HashSet;
use std::fs;
use std::ops::{RangeInclusive};
use itertools::Itertools;
use nom::bytes::complete::{tag};
use nom::character::complete;
use nom::character::complete::{digit1, space0};
use nom::error::Error;
use nom::IResult;


pub fn solve_first_star() -> usize {
    let sensors = parse_input("src/proboscidea_volcanium/input.txt");

    // TODO: Use adjacency lists

    // TODO: optimise using a fitness function

    // TODO: Solve using heuristic for fitness function
    // Will greedy work?
    // heusristic should be based on flow rate, time remaining and distance.


    0
}


pub fn solve_second_star() -> usize {
    let sensors = parse_input("src/proboscidea_volcanium/input.txt");


    0
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
struct Sensor {
    position: Point,
    beacon: Point,
}

impl Sensor {
    fn manhatten_distance(&self) -> usize {
        (self.position.x.abs_diff(self.beacon.x) +
            self.position.y.abs_diff(self.beacon.y)) as usize
    }

    /// Returns the x coordinate detection range for the y coordinate provided
    fn detection_range(&self, y: i64) -> Option<RangeInclusive<i64>> {
        // y distance erosion. e.g. the amount of the manhat dist consumed by the y offset
        let y_offset = self.position.y.abs_diff(y);

        // Calculate range of sensor accounting for y_offset, return None if out of range
        let distance_range = self.manhatten_distance().checked_sub(y_offset as usize)?;

        // construct range
        let start = self.position.x - distance_range as i64;
        let end = self.position.x + distance_range as i64;

        Some(start..=end)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn count_impossible_positions(sensors: Vec<Sensor>, y: i64) -> usize {
    let sensor_ranges = sensors_combined_range(&sensors, y);

    // collect unique beacons filtering on only those in the searched y coordinate
    let beacons = sensors
        .into_iter()
        .filter(|x| x.beacon.y == y)
        .map(|x| x.beacon)
        .collect::<HashSet<_>>();

    // filter beacons further only if they are in the sensors ranges and count
    let num_beacons = beacons
        .into_iter()
        .filter(|b| sensor_ranges.iter().any(|r| r.contains(&b.x)))
        .count();

    sensor_ranges.into_iter().fold(0, |acc, x| x.count()) - num_beacons
}

fn sensors_combined_range(sensors: &Vec<Sensor>, y: i64) -> Vec<RangeInclusive<i64>> {
    let mut raw_sensor_ranges = Vec::<RangeInclusive<i64>>::new();

    for sensor in sensors.iter() {
        if let Some(range) = sensor.detection_range(y) {
            raw_sensor_ranges.push(range);
        }
    }

    // sort ranges prior to coalesce
    raw_sensor_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    // coalesce ranges
    let sensor_ranges = raw_sensor_ranges
        .into_iter()
        .coalesce(|x, y| {
            //println!("x = {:?}, y = {:?}", x, y);
            // check if y is encapsulated by x
            if y.start() <= x.end() {
                if y.end() <= x.end() {
                    Ok(x)
                } else {
                    Ok(*x.start()..=*y.end())
                }
            } else {
                Err((x, y))
            }
        })
        .collect::<Vec<_>>();
    sensor_ranges
}

fn parse_input(file_path: &str) -> Vec::<Sensor> {
    let content = fs::read_to_string(file_path).expect("Input file local to project");

    // parse sensor information
    let mut sensors = Vec::<Sensor>::new();

    for line in content.lines() {
        if let Ok((_, sensor)) = parse_sensor_entry(line) {
            sensors.push(sensor);
        }
    }

    sensors
}


fn parse_sensor_entry(input: &str) -> IResult<&str, Sensor> {
    // Sensor at x=3729579, y=1453415: closest beacon is at x=4078883, y=2522671
    let (remaining, _) = space0(input)?;
    let (remaining, _) = tag("Sensor at")(remaining)?;
    let (remaining, position) = parse_point(remaining)?;
    let (remaining, _) = tag(":")(remaining)?;
    let (remaining, _) = space0(remaining)?;
    let (remaining, _) = tag("closest beacon is at")(remaining)?;
    let (remaining, beacon) = parse_point(remaining)?;

    let s = Sensor {
        position,
        beacon,
    };

    Ok((remaining, s))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    // x=3729579, y=1453415
    let (remaining, _) = space0(input)?;
    let (remaining, _) = tag("x=")(remaining)?;
    let (remaining, x) = parse_i64(remaining)?;
    let (remaining, _) = tag(", y=")(remaining)?;
    let (remaining, y) = parse_i64(remaining)?;

    Ok((remaining, Point { x, y }))
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (rest, num) = digit1(input)?;

    let (_, num) = complete::u64::<_, Error<_>>(num).unwrap();

    Ok((rest, num as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(4827924, solve_first_star());
        assert_eq!(12977110973564, solve_second_star());
    }

    #[test]
    fn test_impossible_positions() {
        let sensors = parse_input("src/beacon_exclusion_zone/test_input.txt");

        let y = 10;

        let result = count_impossible_positions(sensors, y);

        assert_eq!(26, result);
    }

    #[test]
    fn test_beacon_location() {
        let sensors = parse_input("src/beacon_exclusion_zone/test_input.txt");

        let y_range = 0..=20;

        for y in y_range {
            let range = sensors_combined_range(&sensors, y);

            println!("y= {} , what range = {:?}", y, range);

            // TODO: Finish identifying how to rule out other contenders.


            if range.len() > 1 {


                println!("contender = {:?}", range);
            }
        }

        assert_eq!(26, 0);
    }
}