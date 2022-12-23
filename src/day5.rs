use std::convert::TryInto;
use std::{collections::VecDeque, fs::File, io::Read};

use nom::bytes::complete::tag;
use nom::IResult;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}
fn parse_move_with_nom(moves: &str) -> IResult<&str, Move> {
    let (moves, _) = tag("move ")(moves)?;
    let (moves, count) = nom::character::complete::u32(moves)?;
    println!("count:{count}, moves: {moves}");
    let (moves, _) = tag(" from ")(moves)?;
    let (moves, from) = nom::character::complete::u32(moves)?;
    let (moves, _) = tag(" to ")(moves)?;
    let (moves, to) = nom::character::complete::u32(moves)?;
    let (moves, _) = nom::combinator::opt(nom::character::complete::char('\n'))(moves)?;
    Ok((
        moves,
        Move {
            from: from.try_into().unwrap(),
            to: to.try_into().unwrap(),
            count: count.try_into().unwrap(),
        },
    ))
}

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

pub fn part1_with_nom() {
    let mut input = EXAMPLE;
    // let whole_input = read_input();
    // let mut input: &str = &whole_input;

    let mut stacks: Vec<VecDeque<u8>> = Vec::new();

    loop {
        let next_newline_idx = input.find('\n').unwrap();
        let current_line = &input[..next_newline_idx];
        if stacks.is_empty() {
            let n_stacks = (current_line.len() + 1) / 4;
            stacks.resize(n_stacks, VecDeque::new());
        }
        if !current_line.contains('[') {
            input = &input[next_newline_idx + 2..]; // +1 skips the next newline
            break;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..stacks.len() {
            if current_line.as_bytes()[i * 4 + 1] != b' ' {
                stacks[i].push_front(current_line.as_bytes()[i * 4 + 1])
            }
        }

        input = &input[next_newline_idx + 1..];
    }

    while let Ok((next_input, next_move)) = parse_move_with_nom(input) {
        for _ in 0..next_move.count {
            if let Some(crate_) = stacks[next_move.from - 1].pop_back() {
                stacks[next_move.to - 1].push_back(crate_);
            } else {
                panic!("invalid number of crates?")
            }
        }
        input = next_input
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
            let idx = from.len() + i - crates_to_move;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_with_nom() {
        let one_move_no_trailing_newline = "move 1 from 2 to 3";
        match parse_move_with_nom(one_move_no_trailing_newline) {
            Ok((remaining, mov)) => {
                assert_eq!(remaining, "");
                assert_eq!(
                    mov,
                    Move {
                        from: 2,
                        to: 3,
                        count: 1
                    }
                );
            }
            Err(e) => panic!("failed to parse: {e:?}"),
        }
        let one_move_no_trailing_newline = "move 1 from 2 to 3\n";
        match parse_move_with_nom(one_move_no_trailing_newline) {
            Ok((remaining, mov)) => {
                assert_eq!(remaining, "");
                assert_eq!(
                    mov,
                    Move {
                        from: 2,
                        to: 3,
                        count: 1
                    }
                );
            }
            Err(e) => panic!("failed to parse: {e:?}"),
        }
    }
}
