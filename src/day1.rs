use std::{fs::File, io::Read, mem};

type Elf = Vec<u32>;

type Input = Vec<Elf>;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day1.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

fn parse_input(input: &str) -> Input {
    let mut result = Vec::new();

    let mut next_elf = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let finished_elf = mem::take(&mut next_elf);
            result.push(finished_elf);
        } else {
            let v: u32 = line.parse().expect("parsing line");
            next_elf.push(v);
        }
    }
    let finished_elf = mem::take(&mut next_elf);
    result.push(finished_elf);

    result
}

#[allow(unused)]
const EXAMPLE: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

pub fn part1() {
    let input = read_input();
    let elves = parse_input(&input);
    //     let elves = parse_input(
    //
    let max: u32 = elves
        .into_iter()
        .map(|v| v.into_iter().sum())
        .max()
        .unwrap();
    println!("{}", max)
}

pub fn part2() {
    let input = read_input();
    let elves = parse_input(&input);
    let mut elf_totals: Vec<u32> = elves.into_iter().map(|v| v.into_iter().sum()).collect();
    elf_totals.sort();
    println!("{}", elf_totals[elf_totals.len() - 3..].iter().sum::<u32>())
}
