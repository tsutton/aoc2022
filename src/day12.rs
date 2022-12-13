use std::{collections::VecDeque, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day12.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

pub fn part1() {
    // let input = EXAMPLE;
    let input = &read_input();
    // we're going to store this in row-major order i.e. grid[0] is the first row, grid[1] is the second row, etc
    // and mapping chars onto u8 with 'a' will be 0, 'b' 1, etc

    let mut start = (0, 0);
    let mut end = (0, 0);

    let grid: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col_number, b)| {
                    if *b == b'E' {
                        end = (line_number, col_number);
                        b'z' - b'a'
                    } else if *b == b'S' {
                        start = (line_number, col_number);
                        0
                    } else {
                        b - b'a'
                    }
                })
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    // We're going to do this as breadth-first search, finding, for each square, the min number of steps to reach that square (stopping
    // early if we find the end-goal)
    // here's our array of final-answers. Plus "None" is our "haven't visited yet" [could use -1 instead, either is fine]
    let mut min_steps_to_reach: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some(((next_row, next_col), previous_steps)) = queue.pop_front() {
        // println!(
        //     "processing ({},{}) with steps {}",
        //     next_row, next_col, previous_steps
        // );
        if min_steps_to_reach[next_row][next_col].is_some() {
            continue;
        }
        min_steps_to_reach[next_row][next_col] = Some(previous_steps);
        let mut possible_nexts = vec![];
        if next_row > 0 {
            possible_nexts.push((next_row - 1, next_col));
        }
        if next_row < height - 1 {
            possible_nexts.push((next_row + 1, next_col));
        }
        if next_col > 0 {
            possible_nexts.push((next_row, next_col - 1));
        }
        if next_col < width - 1 {
            possible_nexts.push((next_row, next_col + 1));
        }
        // println!("{possible_nexts:?}");
        for (x, y) in possible_nexts {
            if min_steps_to_reach[x][y].is_none() && grid[x][y] <= grid[next_row][next_col] + 1 {
                queue.push_back(((x, y), previous_steps + 1));
            }
        }
        // todo short circuit on end
    }
    println!("{}", min_steps_to_reach[end.0][end.1].unwrap());
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = &read_input();

    let mut end = (0, 0);

    // This time we're gonna do BFS starting from the end, keeping track of the min at elev A each time
    // Grid is setup the same, except we don't care about start any more
    let grid: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col_number, b)| {
                    if *b == b'E' {
                        end = (line_number, col_number);
                        b'z' - b'a'
                    } else if *b == b'S' {
                        0
                    } else {
                        b - b'a'
                    }
                })
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    // same as last time but we're allowed to go down at most one instead of up since we're going backwards
    let mut min_steps_to_reach: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    let mut queue = VecDeque::new();
    // first node to visit is the end
    queue.push_back((end, 0));

    // we'll keep track of the min to any 'a' in this var.
    let mut min_from_an_a = None;

    while let Some(((next_row, next_col), previous_steps)) = queue.pop_front() {
        // println!(
        //     "processing ({},{}) with steps {}",
        //     next_row, next_col, previous_steps
        // );
        if min_steps_to_reach[next_row][next_col].is_some() {
            continue;
        }
        min_steps_to_reach[next_row][next_col] = Some(previous_steps);
        let mut possible_nexts = vec![];
        if next_row > 0 {
            possible_nexts.push((next_row - 1, next_col));
        }
        if next_row < height - 1 {
            possible_nexts.push((next_row + 1, next_col));
        }
        if next_col > 0 {
            possible_nexts.push((next_row, next_col - 1));
        }
        if next_col < width - 1 {
            possible_nexts.push((next_row, next_col + 1));
        }
        // println!("{possible_nexts:?}");
        for (x, y) in possible_nexts {
            // This line is different than part 1
            if min_steps_to_reach[x][y].is_none() && grid[x][y] + 1 >= grid[next_row][next_col] {
                queue.push_back(((x, y), previous_steps + 1));
            }
        }

        if grid[next_row][next_col] == 0 {
            match min_from_an_a {
                None => min_from_an_a = Some(previous_steps),
                Some(x) if x > previous_steps => min_from_an_a = Some(previous_steps),
                _ => {}
            }
        }
    }
    println!("{}", min_from_an_a.unwrap())
}
