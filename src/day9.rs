use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day9.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

const EXAMPLE: &str = "";

pub fn part1() {}

pub fn part2() {}
