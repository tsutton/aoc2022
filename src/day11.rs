#![allow(clippy::bool_to_int_with_if)]

/// I decided not to implement a parser for the input format and instead hardcode the example monkeys
/// and the input monkeys, since it's easier.
/// Part 1 is at the top level, but part 2 needed some re-implementation of the entire monkey idea so
/// it's in a separate module.
/// I think it could be combined by making Monkey generic over sufficiently numeric types but eh
use std::fmt::Display;

struct Monkey {
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    items: Vec<u64>,
}

impl Monkey {
    // return (index, new worry level)
    fn inspect_first_item(&mut self) -> Option<(usize, u64)> {
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

#[allow(unused)]
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

pub use part2::part2;

mod part2 {
    use std::ops::{Add, Mul, Rem};

    // For part 2, we need to exploit the fact that the tests are all "number mod n == 0"
    // and (not necessary for a strategy like this to work, but makes it simpler) all the n are prime
    // Ultimately the full operations we do on the worry-levels are addition, multiplication, and remainder mod n
    // and it's a nify fact (called the Chinese Remainder Theorem, or CRT), that whether we track the actual number,
    // or just track the number mod n for all the n at the same time, the results are the same.
    // (Note, before doing it this way, I naively tried using u64 and it still wasn't enough precision)
    #[derive(Clone)]
    struct CrtNumber {
        // pairs (value, modulus)
        // e.g. we'd read [(1,5), (2,7)] as a number wihch is 1mod5 and 2 mod 7
        data: Vec<(u32, u32)>,
    }

    impl Add<u32> for CrtNumber {
        type Output = CrtNumber;

        fn add(self, rhs: u32) -> Self::Output {
            CrtNumber {
                data: self
                    .data
                    .into_iter()
                    .map(|(value, modulus)| ((value + rhs) % modulus, modulus))
                    .collect(),
            }
        }
    }

    impl Mul<u32> for CrtNumber {
        type Output = CrtNumber;

        fn mul(self, rhs: u32) -> Self::Output {
            CrtNumber {
                data: self
                    .data
                    .into_iter()
                    .map(|(value, modulus)| ((value * rhs) % modulus, modulus))
                    .collect(),
            }
        }
    }

    impl Mul<CrtNumber> for CrtNumber {
        type Output = CrtNumber;

        fn mul(self, rhs: CrtNumber) -> Self::Output {
            CrtNumber {
                data: self
                    .data
                    .into_iter()
                    .zip(rhs.data.into_iter())
                    .map(|((value1, modulus1), (value2, _modulus2))| {
                        ((value1 * value2) % modulus1, modulus1)
                    })
                    .collect(),
            }
        }
    }

    impl Rem<u32> for &CrtNumber {
        type Output = u32;

        fn rem(self, rhs: u32) -> Self::Output {
            for (value, modulus) in self.data.iter() {
                if *modulus == rhs {
                    return *value;
                }
            }
            unimplemented!("modulus of {rhs}");
        }
    }

    const MODULI: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];

    struct Monkey {
        operation: Box<dyn Fn(CrtNumber) -> CrtNumber>,
        test: Box<dyn Fn(CrtNumber) -> usize>,
        items: Vec<CrtNumber>,
    }

    impl Monkey {
        // return (index, new worry level)
        fn inspect_first_item(&mut self) -> Option<(usize, CrtNumber)> {
            if self.items.is_empty() {
                return None;
            }
            let worry = self.items.remove(0);
            let worry = (self.operation)(worry);
            let idx = (self.test)(worry.clone());
            Some((idx, worry))
        }
    }

    fn to_crt_num(value: u32) -> CrtNumber {
        CrtNumber {
            data: MODULI.iter().map(|m| (value % *m, *m)).collect(),
        }
    }

    #[allow(unused)]
    fn example_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                operation: Box::new(|worry| worry * 19),
                test: Box::new(|worry| if &worry % 23 == 0 { 2 } else { 3 }),
                items: vec![79, 98].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                operation: Box::new(|worry| worry + 6),
                test: Box::new(|worry| if &worry % 19 == 0 { 2 } else { 0 }),
                items: vec![54, 65, 75, 74].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                operation: Box::new(|worry| worry.clone() * worry),
                test: Box::new(|worry| if &worry % 13 == 0 { 1 } else { 3 }),
                items: vec![79, 60, 97].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                operation: Box::new(|worry| worry + 3),
                test: Box::new(|worry| if &worry % 17 == 0 { 0 } else { 1 }),
                items: vec![74].into_iter().map(to_crt_num).collect(),
            },
        ]
    }

    fn real_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                // 0
                operation: Box::new(|worry| worry * 7),
                test: Box::new(|worry| if &worry % 17 == 0 { 5 } else { 3 }),
                items: vec![54, 89, 94].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                // 1
                operation: Box::new(|worry| worry + 4),
                test: Box::new(|worry| if &worry % 3 == 0 { 0 } else { 3 }),
                items: vec![66, 71].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                // 2
                operation: Box::new(|worry| worry + 2),
                test: Box::new(|worry| if &worry % 5 == 0 { 7 } else { 4 }),
                items: vec![76, 55, 80, 55, 55, 96, 78]
                    .into_iter()
                    .map(to_crt_num)
                    .collect(),
            },
            Monkey {
                // 3
                operation: Box::new(|worry| worry + 7),
                test: Box::new(|worry| if &worry % 7 == 0 { 5 } else { 2 }),
                items: vec![93, 69, 76, 66, 89, 54, 59, 94]
                    .into_iter()
                    .map(to_crt_num)
                    .collect(),
            },
            Monkey {
                // 4
                operation: Box::new(|worry| worry * 17),
                test: Box::new(|worry| if &worry % 11 == 0 { 1 } else { 6 }),
                items: vec![80, 54, 58, 75, 99]
                    .into_iter()
                    .map(to_crt_num)
                    .collect(),
            },
            Monkey {
                // 5
                operation: Box::new(|worry| worry + 8),
                test: Box::new(|worry| if &worry % 19 == 0 { 2 } else { 7 }),
                items: vec![69, 70, 85, 83].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                // 6
                operation: Box::new(|worry| worry + 6),
                test: Box::new(|worry| if &worry % 2 == 0 { 0 } else { 1 }),
                items: vec![89].into_iter().map(to_crt_num).collect(),
            },
            Monkey {
                // 7
                operation: Box::new(|worry| worry.clone() * worry),
                test: Box::new(|worry| if &worry % 13 == 0 { 6 } else { 4 }),
                items: vec![62, 80, 58, 57, 93, 56]
                    .into_iter()
                    .map(to_crt_num)
                    .collect(),
            },
        ]
    }
    pub fn part2() {
        let mut monkeys = real_monkeys();
        // let mut monkeys = example_monkeys();
        let mut number_of_inspections = vec![0u64; monkeys.len()];
        for _round in 0..10000 {
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
}
