use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day8.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"30373
25512
65332
33549
35390
";

pub fn part1() {}
pub fn part2() {}
