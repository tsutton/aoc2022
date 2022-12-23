use std::collections::{BTreeSet, HashMap};
use std::convert::TryInto;
use std::fmt::Display;
#[allow(unused)]
use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day14.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

#[derive(Debug, Clone, Default)]
struct Column {
    occupied_spaces: BTreeSet<i32>,
}

impl Column {
    fn least_occupied_space_after(&self, after: i32) -> Option<i32> {
        self.occupied_spaces.range(after..).next().copied()
    }

    fn insert(&mut self, value: i32) {
        self.occupied_spaces.insert(value);
    }
}

mod part1 {

    use super::*;

    struct Cave {
        data: HashMap<usize, Column>,
    }

    fn parse_coordinate(input: &str) -> (usize, usize) {
        let node_coords: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();
        (node_coords[0], node_coords[1])
    }

    impl Cave {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }

        fn add_path(&mut self, path: &str) {
            let node_strs: Vec<&str> = path.split(" -> ").collect();
            assert!(node_strs.len() > 1);
            for (i, window) in node_strs.windows(2).enumerate() {
                let (start_x, start_y) = parse_coordinate(window[0]);
                let (end_x, end_y) = parse_coordinate(window[1]);
                if i == 0 {
                    self.add_square(start_x, start_y);
                }
                if start_x == end_x {
                    let (start_y, end_y) = (start_y.min(end_y), start_y.max(end_y));
                    for y in start_y..=end_y {
                        self.add_square(start_x, y);
                    }
                } else if start_y == end_y {
                    let (start_x, end_x) = (start_x.min(end_x), start_x.max(end_x));
                    for x in start_x..=end_x {
                        self.add_square(x, start_y);
                    }
                }
            }
        }

        fn add_square(&mut self, start_x: usize, start_y: usize) {
            self.data
                .entry(start_x)
                .or_default()
                .insert(start_y.try_into().unwrap());
        }

        fn drop_sand(&mut self) -> Option<bool> {
            self.drop_sand_from(500, 0)
        }
        // returns Some(true) if sand fell into the abyss, Some(false) if sand fell, but not into the abyss
        // and None if sand can't fall from here because (column_idx, row_index is already occupied)
        fn drop_sand_from(&mut self, column_idx: usize, row_idx: i32) -> Option<bool> {
            let next_place_in_current_column = self
                .data
                .entry(column_idx)
                .or_default()
                .least_occupied_space_after(row_idx);
            match next_place_in_current_column {
                None => Some(true),
                Some(row) if row > row_idx => {
                    // try to drop sand from left
                    let drop_left_attempt = self.drop_sand_from(column_idx - 1, row);
                    if drop_left_attempt.is_some() {
                        return drop_left_attempt;
                    }
                    let drop_right_attempt = self.drop_sand_from(column_idx + 1, row);
                    if drop_right_attempt.is_some() {
                        return drop_right_attempt;
                    }
                    self.data.entry(column_idx).or_default().insert(row - 1);
                    Some(false)
                }
                _ => None,
            }
        }
    }

    impl Display for Cave {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut columns: Vec<_> = self.data.keys().copied().collect();
            columns.sort();
            let max_row = self
                .data
                .values()
                .map(|col| col.occupied_spaces.last().copied().unwrap_or_default())
                .max()
                .unwrap();
            for row in 0..=max_row {
                for column in columns[0]..=columns.last().copied().unwrap() {
                    if !self.data.contains_key(&column) {
                        write!(f, ".")?;
                    } else if self.data[&column].occupied_spaces.contains(&row) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f,)?;
            }
            Ok(())
        }
    }

    pub fn part1() {
        // let input = EXAMPLE;
        let input = read_input();
        let mut cave = Cave::new();
        for line in input.lines() {
            cave.add_path(line);
        }
        // println!("{cave}");
        let mut count = 0;
        while let Some(x) = cave.drop_sand() {
            if !x {
                // println!("{cave}");
                count += 1;
            } else {
                println!("{count}");
                return;
            }
        }
        panic!("drop_sand returned none");
    }
}

pub use part1::part1;

pub fn part2() {}
