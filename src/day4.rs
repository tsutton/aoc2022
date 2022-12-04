use std::{collections::HashSet, fs::File, io::Read, ops::Range};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day4.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut count = 0;
    for line in input.lines() {
        let (first_raw_range, second_raw_range) = line.split_once(',').unwrap();

        // This use of Range is not really right since we're using it as closed, not half-open, but
        // it's a convenient (start, end) struct
        let (f1, f2) = first_raw_range.split_once('-').unwrap();
        let first_range: Range<u32> = f1.parse::<u32>().unwrap()..f2.parse::<u32>().unwrap();

        let (f1, f2) = second_raw_range.split_once('-').unwrap();
        let second_range: Range<u32> = f1.parse::<u32>().unwrap()..f2.parse::<u32>().unwrap();
        if (first_range.start <= second_range.start && first_range.end >= second_range.end)
            || (first_range.start >= second_range.start && first_range.end <= second_range.end)
        {
            count += 1;
        }
    }
    println!("{count}")
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut count = 0;
    for line in input.lines() {
        let (first_raw_range, second_raw_range) = line.split_once(',').unwrap();

        let (f1, f2) = first_raw_range.split_once('-').unwrap();
        // This use of Range is not really right since we're using it as closed, not half-open, but
        // it's a convenient (start, end) struct
        let first_range: Range<u32> = f1.parse::<u32>().unwrap()..f2.parse::<u32>().unwrap();

        let (f1, f2) = second_raw_range.split_once('-').unwrap();
        let second_range: Range<u32> = f1.parse::<u32>().unwrap()..f2.parse::<u32>().unwrap();

        // if two ranges overlap, then the start of one must be inside the other
        if (
            // check if first.start is inside second
            first_range.start >= second_range.start && first_range.start <= second_range.end
        ) || (
            // check if second.start is inside first
            second_range.start >= first_range.start && second_range.start <= first_range.end
        ) {
            println!("{line}");
            count += 1;
        }
    }
    println!("{count}")
}
