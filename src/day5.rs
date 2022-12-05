use std::{
    collections::{HashSet, VecDeque},
    convert::TryInto,
    fs::File,
    io::Read,
    ops::Range,
};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day5.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut lines = input.lines().peekable();
    let n_stacks = (lines.peek().unwrap().len() + 1) / 4;
    let mut stacks: Vec<VecDeque<u8>> = Vec::with_capacity(n_stacks);
    stacks.resize(n_stacks, VecDeque::new());
    while lines.peek().unwrap().contains('[') {
        let line = lines.next().unwrap().as_bytes();
        for i in 0..n_stacks {
            if line[i * 4 + 1] != b' ' {
                stacks[i].push_front(line[i * 4 + 1])
            }
        }
    }
    lines.next(); // numbers
    lines.next(); // empty
    for line in lines {
        // println!("line: {line}");
        let parts: Vec<&str> = line.split(' ').collect();
        let crates_to_move: u32 = parts[1].parse().unwrap();
        let stack_from: usize = parts[3].parse().unwrap();
        let stack_to: usize = parts[5].parse().unwrap();
        for _ in 0..crates_to_move {
            if let Some(crate_) = stacks[stack_from - 1].pop_back() {
                stacks[stack_to - 1].push_back(crate_);
            } else {
                panic!("invalid number of crates?")
            }
        }
    }
    let code: Vec<char> = stacks
        .iter()
        .map(|stack| stack[stack.len() - 1].into())
        .collect();
    println!("{code:?}");
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut lines = input.lines().peekable();
    let n_stacks = (lines.peek().unwrap().len() + 1) / 4;
    let mut stacks: Vec<VecDeque<u8>> = Vec::with_capacity(n_stacks);
    stacks.resize(n_stacks, VecDeque::new());
    while lines.peek().unwrap().contains('[') {
        let line = lines.next().unwrap().as_bytes();
        for i in 0..n_stacks {
            if line[i * 4 + 1] != b' ' {
                stacks[i].push_front(line[i * 4 + 1])
            }
        }
    }
    lines.next(); // numbers
    lines.next(); // empty
    for line in lines {
        // println!("line: {line}");
        let parts: Vec<&str> = line.split(' ').collect();
        let crates_to_move: usize = parts[1].parse().unwrap();
        let stack_from: usize = parts[3].parse().unwrap();
        let stack_to: usize = parts[5].parse().unwrap();
        for i in 0..crates_to_move {
            let from = &stacks[stack_from - 1];
            let idx = from.len() + (i as usize) - crates_to_move;
            let c = from[idx];
            let to = &mut stacks[stack_to - 1];
            to.push_back(c);
        }
        let new_from_len = stacks[stack_from - 1].len() - crates_to_move;
        stacks[stack_from - 1].truncate(new_from_len);
    }
    let code: Vec<char> = stacks
        .iter()
        .map(|stack| stack[stack.len() - 1].into())
        .collect();
    println!("{code:?}");
}
