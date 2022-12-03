use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day2.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"A Y
B X
C Z";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn defeats(&self, other: &Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
                | (Self::Paper, Self::Rock)
        )
    }

    fn move_which_ties(&self) -> Move {
        *self
    }

    fn move_which_defeats(&self) -> Move {
        match self {
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
            Self::Paper => Self::Scissors,
        }
    }

    fn move_which_loses(&self) -> Move {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
        }
    }

    fn innate_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .map(|line| {
            let chars: Vec<u8> = line.bytes().collect();
            let m1 = match chars[0] {
                b'A' => Move::Rock,
                b'B' => Move::Paper,
                b'C' => Move::Scissors,
                x => panic!("unexpected p1 move {}", x),
            };
            let m2 = match chars[2] {
                b'X' => Move::Rock,
                b'Y' => Move::Paper,
                b'Z' => Move::Scissors,
                x => panic!("unexpected p2 move {}", x),
            };
            (m1, m2)
        })
        .collect()
}

fn part1_eval_play((their_play, my_play): (Move, Move)) -> u32 {
    let innate_value = match my_play {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let victory_value = if their_play.defeats(&my_play) {
        0
    } else if my_play.defeats(&their_play) {
        6
    } else {
        3
    };
    victory_value + innate_value
}

pub fn part1() {
    // let input = EXAMPLE;
    let input = &read_input();
    let parsed = parse_input(input);
    for p in &parsed {
        println!("{:?} - {:?}", p.0, p.1);
    }
    let result: u32 = parsed.into_iter().map(part1_eval_play).sum();
    println!("{}", result)
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = &read_input();
    let score: u32 = input
        .lines()
        .map(|line| {
            let chars: Vec<u8> = line.bytes().collect();
            let opponents_move = match chars[0] {
                b'A' => Move::Rock,
                b'B' => Move::Paper,
                b'C' => Move::Scissors,
                x => panic!("unexpected p1 move {}", x),
            };
            match chars[2] {
                b'X' => opponents_move.move_which_loses().innate_score(),
                b'Y' => opponents_move.move_which_ties().innate_score() + 3,
                b'Z' => opponents_move.move_which_defeats().innate_score() + 6,
                x => panic!("unexpected result {}", x),
            }
        })
        .sum();
    println!("{}", score)
}
