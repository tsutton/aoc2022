use std::{collections::HashSet, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day3.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut sum = 0;
    for line in input.lines() {
        let half_len = line.len() / 2;
        let first_half: HashSet<char> = line[..half_len].chars().collect();
        let second_half: HashSet<char> = line[half_len..].chars().collect();
        let intersection = first_half
            .intersection(&second_half)
            .into_iter()
            .collect::<Vec<_>>()[0];
        // println!("{intersection}");
        let value = match intersection {
            'a'..='z' => intersection.to_digit(36).unwrap() - 9,
            'A'..='Z' => intersection.to_digit(36).unwrap() + 17,
            _ => unreachable!(),
        };
        // println!("{value}");
        sum += value;
    }
    println!("{sum}");
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut sum = 0;
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let (elf1, elf2, elf3): (HashSet<char>, HashSet<char>, HashSet<char>) = (
            lines.next().unwrap().chars().collect(),
            lines.next().unwrap().chars().collect(),
            lines.next().unwrap().chars().collect(),
        );
        let common_element = *elf1
            .intersection(&elf2)
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&elf3)
            .collect::<Vec<_>>()[0];
        // println!("{common_element}");
        let value = match common_element {
            'a'..='z' => common_element.to_digit(36).unwrap() - 9,
            'A'..='Z' => common_element.to_digit(36).unwrap() + 17,
            _ => unreachable!(),
        };
        // println!("{value}");
        sum += value;
    }
    println!("{sum}");
}
