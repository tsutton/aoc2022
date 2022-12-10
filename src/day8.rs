use std::{fs::File, io::Read, ops::Index};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day8.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

struct Map {
    width: usize,
    height: usize,

    // stored in row-major order, i.e. the map:
    // 123
    // 456
    // 789
    // is stored as 1,2,3,4,5,6,7,8,9
    data: Vec<u32>,
}

impl Map {
    fn from_text(text: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();
        for (i, line) in text.lines().enumerate() {
            if i == 0 {
                width = line.len();
            }

            // last value of i sets final height
            height = i + 1;

            for c in line.chars() {
                data.push(c.to_digit(10).expect("text contains digits from 0 to 9"));
            }
        }
        Self {
            width,
            height,
            data,
        }
    }

    fn visible_count(&self) -> usize {
        let mut visible_tracking_matrix = vec![0; self.width * self.height];

        // first go through each row from 0 to height, checking the row from the left and right sides
        for row in 0..self.height {
            let mut max_height_from_left = self[(0, row)];
            for col in 0..self.width {
                if col == 0 || max_height_from_left < self[(col, row)] {
                    visible_tracking_matrix[self.width * row + col] = 1;
                    max_height_from_left = self[(col, row)];
                }
            }

            let mut max_height_from_right = self[(self.width - 1, row)];
            for col_from_right in 0..self.width {
                let col = self.width - 1 - col_from_right;
                if col == self.width - 1 || self[(col, row)] > max_height_from_right {
                    visible_tracking_matrix[self.width * row + col] = 1;
                    max_height_from_right = self[(col, row)];
                }
            }
        }

        // now check from top to bottom and bottom to top within each column
        for col in 0..self.width {
            let mut max_height_from_above = self[(col, 0)];
            for row in 0..self.height {
                if row == 0 || self[(col, row)] > max_height_from_above {
                    visible_tracking_matrix[self.width * row + col] = 1;
                    max_height_from_above = self[(col, row)];
                }
            }

            let mut max_height_from_below = self[(col, self.height - 1)];
            for row_from_bottom in 0..self.height {
                let row = self.height - 1 - row_from_bottom;
                if row == self.height - 1 || max_height_from_below < self[(col, row)] {
                    visible_tracking_matrix[self.width * row + col] = 1;
                    max_height_from_below = self[(col, row)]
                }
            }
        }

        // for i in 0..self.height {
        //     println!(
        //         "{:?}",
        //         &visible_tracking_matrix[i * self.width..(i + 1) * self.width]
        //     )
        // }
        visible_tracking_matrix.iter().sum()
    }

    fn max_scenic_score(&self) -> usize {
        // what we want, for each tree, is to find the length of the runs on each of the four sides.
        // The naive way would be to go through each tree and find the runs for that tree, but we can do better.

        // vec of (left_run, right_run, top_run, bottom_run)
        let mut run_tracking_matrix = vec![(0, 0, 0, 0); self.width * self.height];
        let index = |col, row| row * self.width + col;

        // first go through each row from 0 to height, checking the row from the left and right sides
        for row in 0..self.height {
            for col in 0..self.width {
                let first_bigger_col_offset =
                    (1..col).find(|run_length| self[(col - run_length, row)] >= self[(col, row)]);
                let left_run_length = match first_bigger_col_offset {
                    None => {
                        // there were no trees to the right that were bigger:
                        col
                    }
                    Some(t) => {
                        // row[col - t] is bigger
                        t
                    }
                };
                run_tracking_matrix[index(col, row)].0 = left_run_length;

                let first_bigger_col_offset = (1..(self.width - col))
                    .find(|run_length| self[(col + run_length, row)] >= self[(col, row)]);
                let right_run_length = match first_bigger_col_offset {
                    None => {
                        // there were no trees to the right that were bigger:
                        self.width - col - 1
                    }
                    Some(t) => {
                        // row[col + t] is bigger
                        t
                    }
                };
                run_tracking_matrix[index(col, row)].1 = right_run_length;
            }
        }

        // next, go through each column, first top to bottom then bottom to top
        for col in 0..self.width {
            for row in 0..self.height {
                let first_bigger_row_offset =
                    (1..row).find(|run_length| self[(col, row - run_length)] >= self[(col, row)]);
                let top_run_length = match first_bigger_row_offset {
                    None => {
                        // there were no trees to the top that were bigger:
                        row
                    }
                    Some(t) => {
                        // col[row - t] is bigger
                        t
                    }
                };
                run_tracking_matrix[index(col, row)].2 = top_run_length;

                let first_bigger_row_offset = (1..(self.height - row))
                    .find(|run_length| self[(col, row + run_length)] >= self[(col, row)]);
                let bottom_run_length = match first_bigger_row_offset {
                    None => {
                        // there were no trees below that were bigger:
                        self.height - row - 1
                    }
                    Some(t) => {
                        // col[row + t] is bigger
                        t
                    }
                };
                run_tracking_matrix[index(col, row)].3 = bottom_run_length;
            }
        }

        // for i in 0..self.height {
        //     println!(
        //         "{:?}",
        //         &run_tracking_matrix[i * self.width..(i + 1) * self.width]
        //     )
        // }

        run_tracking_matrix
            .iter()
            .map(|(a, b, c, d)| a * b * c * d)
            .max()
            .unwrap()
    }
}

/// Index is 0-indexed, (x,y) from the top left
impl Index<(usize, usize)> for Map {
    type Output = u32;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.data[row * self.width + col]
    }
}

#[allow(unused)]
const EXAMPLE: &str = r"30373
25512
65332
33549
35390
";

pub fn part1() {
    // let input = EXAMPLE;
    let input = &read_input();
    let map = Map::from_text(input);
    let i = map.visible_count();
    println!("{i}");
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = &read_input();
    let map = Map::from_text(input);
    let i = map.max_scenic_score();
    println!("{i}");
}
