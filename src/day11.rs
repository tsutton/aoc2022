use std::{fmt::Display, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day11.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"";

struct Monkey {
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> usize>,
    items: Vec<u32>,
}

impl Monkey {
    // return (index, new worry level)
    fn inspect_first_item(&mut self) -> Option<(usize, u32)> {
        if self.items.is_empty() {
            return None;
        }
        let worry = self.items.remove(0);
        let worry = (self.operation)(worry);
        let worry = worry / 3;
        let idx = (self.test)(worry);
        Some((idx, worry))
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

fn example_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            operation: Box::new(|worry| worry * 19),
            test: Box::new(|worry| if worry % 23 == 0 { 2 } else { 3 }),
            items: vec![79, 98],
        },
        Monkey {
            operation: Box::new(|worry| worry + 6),
            test: Box::new(|worry| if worry % 19 == 0 { 2 } else { 0 }),
            items: vec![54, 65, 75, 74],
        },
        Monkey {
            operation: Box::new(|worry| worry * worry),
            test: Box::new(|worry| if worry % 13 == 0 { 1 } else { 3 }),
            items: vec![79, 60, 97],
        },
        Monkey {
            operation: Box::new(|worry| worry + 3),
            test: Box::new(|worry| if worry % 17 == 0 { 0 } else { 1 }),
            items: vec![74],
        },
    ]
}

fn real_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            // 0
            operation: Box::new(|worry| worry * 7),
            test: Box::new(|worry| if worry % 17 == 0 { 5 } else { 3 }),
            items: vec![54, 89, 94],
        },
        Monkey {
            // 1
            operation: Box::new(|worry| worry + 4),
            test: Box::new(|worry| if worry % 3 == 0 { 0 } else { 3 }),
            items: vec![66, 71],
        },
        Monkey {
            // 2
            operation: Box::new(|worry| worry + 2),
            test: Box::new(|worry| if worry % 5 == 0 { 7 } else { 4 }),
            items: vec![76, 55, 80, 55, 55, 96, 78],
        },
        Monkey {
            // 3
            operation: Box::new(|worry| worry + 7),
            test: Box::new(|worry| if worry % 7 == 0 { 5 } else { 2 }),
            items: vec![93, 69, 76, 66, 89, 54, 59, 94],
        },
        Monkey {
            // 4
            operation: Box::new(|worry| worry * 17),
            test: Box::new(|worry| if worry % 11 == 0 { 1 } else { 6 }),
            items: vec![80, 54, 58, 75, 99],
        },
        Monkey {
            // 5
            operation: Box::new(|worry| worry + 8),
            test: Box::new(|worry| if worry % 19 == 0 { 2 } else { 7 }),
            items: vec![69, 70, 85, 83],
        },
        Monkey {
            // 6
            operation: Box::new(|worry| worry + 6),
            test: Box::new(|worry| if worry % 2 == 0 { 0 } else { 1 }),
            items: vec![89],
        },
        Monkey {
            // 7
            operation: Box::new(|worry| worry * worry),
            test: Box::new(|worry| if worry % 13 == 0 { 6 } else { 4 }),
            items: vec![62, 80, 58, 57, 93, 56],
        },
    ]
}

pub fn part1() {
    let mut monkeys = real_monkeys();
    let mut number_of_inspections = vec![0; monkeys.len()];
    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            while let Some((idx, worry)) = monkeys[monkey_idx].inspect_first_item() {
                monkeys[idx].items.push(worry);
                number_of_inspections[monkey_idx] += 1;
            }
        }
        // for monkey_idx in 0..monkeys.len() {
        //     println!(
        //         "Round {_round} Monkey {monkey_idx}: {}",
        //         monkeys[monkey_idx]
        //     );
        // }
    }
    let (first, second) = number_of_inspections
        .iter()
        .fold((0, 0), |(first, second), next| {
            if next >= &first {
                (*next, first)
            } else if next > &second {
                (first, *next)
            } else {
                (first, second)
            }
        });
    println!("{}", first * second);
}

pub fn part2() {}
