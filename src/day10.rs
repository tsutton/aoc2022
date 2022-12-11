use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day10.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    // panics on failure, coz AoC laziness
    fn from_str(instruction: &str) -> Self {
        match &instruction[0..4] {
            "noop" => Self::Noop,
            "addx" => {
                let value = instruction[5..].parse().unwrap();
                Self::Addx(value)
            }
            _ => unreachable!(),
        }
    }
}

struct Cpu {
    x: i32,
}

impl Cpu {
    // returns number of cycles it took
    fn execute_instruction(&mut self, instruction: &Instruction) -> i32 {
        match instruction {
            Instruction::Noop => 1,
            Instruction::Addx(x) => {
                self.x += x;
                2
            }
        }
    }
}

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut cpu = Cpu { x: 1 };
    let mut cycle = 1;
    let mut total_signal_strength = 0;
    for line in input.lines() {
        let instruction = Instruction::from_str(line);
        let old_x = cpu.x;
        let new_cycle = cpu.execute_instruction(&instruction) + cycle;
        if new_cycle % 40 > 20 && cycle % 40 <= 20 {
            println!(
                "on cycle {} got reg value {} (instruction was {:?})",
                new_cycle, old_x, instruction
            );
            total_signal_strength += (cycle - cycle % 40 + 20) * old_x
        }
        cycle = new_cycle;
    }
    println!("{}", total_signal_strength)
}

pub fn part2() {}

#[allow(unused)]
const EXAMPLE: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
