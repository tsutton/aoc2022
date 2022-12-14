use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day15.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Sensor {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Beacon {
    x: i64,
    y: i64,
}

fn parse_line(line: &str) -> (Sensor, Beacon) {
    let line = &line[12..]; // remove "Sensor at x=" (len 12)

    let comma = line.find(',').unwrap();
    let sensor_x = line[..comma].parse().unwrap();

    let line = &line[comma + 4..]; // remove // ", y=" (len 4)
    let colon = line.find(':').unwrap();
    let sensor_y = line[..colon].parse().unwrap();

    let line = &line[colon + 25..]; // remove ": closest beacon is at x=" (25)
    let comma = line.find(',').unwrap();
    let beacon_x = line[..comma].parse().unwrap();

    let line = &line[comma + 4..]; // remove // ", y=" (len 4)
    let beacon_y = line.parse().unwrap();

    (
        Sensor {
            x: sensor_x,
            y: sensor_y,
        },
        Beacon {
            x: beacon_x,
            y: beacon_y,
        },
    )
}

// returns a pair (start_x, end_x) such that there are no beacons in range start..end (exclusive on end, inclusive on start)
// returns (0,0) if empty
fn no_beacon_spaces_with_y_from_pair(sensor: &Sensor, beacon: &Beacon, y: i64) -> (i64, i64) {
    let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
    let distance_to_row = (sensor.y - y).abs();
    if distance_to_row > distance {
        (0, 0)
    } else {
        let remaining_distance = distance - distance_to_row; // >= 0
        (
            sensor.x - remaining_distance,
            sensor.x + remaining_distance + 1,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_beacon() {
        let sensor = Sensor { x: 8, y: 7 };
        let beacon = Beacon { x: 2, y: 10 };
        assert_eq!(
            no_beacon_spaces_with_y_from_pair(&sensor, &beacon, 10),
            (2, 15)
        )
    }
}

pub fn part1() {
    // let input = EXAMPLE;
    // let row = 10;

    let input = read_input();
    let row = 2000000;

    let sensor_beacon_pairs: Vec<_> = input.lines().map(parse_line).collect();

    let combined_intervals = intervals_of_no_beacon_in_y(&sensor_beacon_pairs, row);

    let total_length: i64 = combined_intervals.iter().map(|l| l.1 - l.0).sum();
    let beacons_in_row: HashSet<_> = sensor_beacon_pairs
        .iter()
        .map(|(_, beacon)| beacon)
        .filter(|beacon| beacon.y == row)
        .collect();
    // println!("{beacons_in_row:?}");

    let count_beacons_in_row = i64::try_from(beacons_in_row.len()).unwrap();

    println!(
        "{total_length} - {count_beacons_in_row} = {}",
        total_length - count_beacons_in_row
    );
}

fn intervals_of_no_beacon_in_y(pairs: &[(Sensor, Beacon)], y: i64) -> Vec<(i64, i64)> {
    let mut uncombined_intervals: Vec<_> = pairs
        .iter()
        .map(|(sensor, beacon)| no_beacon_spaces_with_y_from_pair(sensor, beacon, y))
        .filter(|t| t != &(0, 0))
        .collect();

    uncombined_intervals.sort_by_key(|interval| interval.0);
    let mut combined_intervals = vec![uncombined_intervals[0]];
    for interval in uncombined_intervals[1..].iter() {
        // check if interval overlaps with last interval in combined
        // if so, union them, if not append
        let current_len = combined_intervals.len();
        let previous = combined_intervals[current_len - 1];
        if interval.0 <= previous.1 {
            combined_intervals[current_len - 1].1 = interval.1.max(previous.1);
        } else {
            combined_intervals.push((interval.0, interval.1));
        }
    }
    combined_intervals
}

// given a slice of disjoint intervals (half-open ) sorted by starting point, and another interval,
// return None if the given single interval is completely contained in the list of intevals,
// and Some(x) if the given interval is not contained, where x is the smallest value in the given interval
// not contained in the list
fn sorted_intervals_point_between(intervals: &[(i64, i64)], min: i64, max: i64) -> Option<i64> {
    // find the first interval whose start is >= min
    match intervals.binary_search_by_key(&min, |interval| interval.0) {
        Ok(idx) => {
            // intervals[idx].0 == min
            if intervals[idx].1 >= max {
                None
            } else {
                Some(intervals[idx].1)
            }
        }
        Err(idx) => {
            // either intervals[idx].0 > min and intervals[idx-1].0 < min, or
            // idx could be 0 or intervals.len()
            if idx == 0 {
                Some(min)
            } else if intervals[idx - 1].1 >= max {
                None
            } else {
                Some(intervals[idx - 1].1)
            }
        }
    }
}

pub fn part2() {
    // first idea: brute force, iterate through all 4000000^2 places.
    // slightly faster idea using the building block in part 1: iterate only through x-values
    // and for each value, do the part-1 equivalent. Let's try that.
    // let input = EXAMPLE;
    // let min_coord = 0;
    // let max_coord = 20;

    let input = read_input();
    let min_coord = 0;
    let max_coord = 4_000_000;

    let sensor_beacon_pairs: Vec<_> = input.lines().map(parse_line).collect();

    for y in min_coord..max_coord {
        let intervals = intervals_of_no_beacon_in_y(&sensor_beacon_pairs, y);
        if let Some(x) = sorted_intervals_point_between(&intervals, min_coord, max_coord) {
            println!("({x}, {y}) => {}", x * 4_000_000 + y);
            return;
        }
    }
}
