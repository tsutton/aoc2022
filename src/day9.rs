use std::{collections::HashSet, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day9.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_position: (i32, i32) = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);

    visited_positions.insert(tail_position);

    for line in input.lines() {
        let direction = match line.as_bytes()[0] {
            b'R' => (1, 0),
            b'L' => (-1, 0),
            b'U' => (0, 1),
            b'D' => (0, -1),
            x => panic!("found unexpected direction {x}"),
        };
        let amount: i32 = line[2..].parse().unwrap();
        for _ in 0..amount {
            head_position = (head_position.0 + direction.0, head_position.1 + direction.1);
            tail_position = next_tail_position(head_position, tail_position);
            visited_positions.insert(tail_position);
        }
    }

    println!("{}", visited_positions.len());
}

fn next_tail_position((head_x, head_y): (i32, i32), (tail_x, tail_y): (i32, i32)) -> (i32, i32) {
    match (head_x - tail_x, head_y - tail_y) {
        (0, 0) | (0, 1) | (1, 0) | (-1, 0) | (0, -1) => (tail_x, tail_y),
        (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (tail_x, tail_y),

        (0, 2) => (tail_x, tail_y + 1),
        (0, -2) => (tail_x, tail_y - 1),
        (2, 0) => (tail_x + 1, tail_y),
        (-2, 0) => (tail_x - 1, tail_y),

        (2, 1) | (1, 2) => (tail_x + 1, tail_y + 1),
        (-2, -1) | (-1, -2) => (tail_x - 1, tail_y - 1),

        (-2, 1) => (tail_x - 1, tail_y + 1),
        (2, -1) => (tail_x + 1, tail_y - 1),
        (1, -2) => (tail_x + 1, tail_y - 1),
        (-1, 2) => (tail_x - 1, tail_y + 1),

        // +-2, +-2 positions only occur in part 2
        (2, 2) => (tail_x + 1, tail_y + 1),
        (2, -2) => (tail_x + 1, tail_y - 1),
        (-2, 2) => (tail_x - 1, tail_y + 1),
        (-2, -2) => (tail_x - 1, tail_y - 1),

        x => panic!("invalid head-tail diff ({}, {})", x.0, x.1),
    }
}

pub fn part2() {
    // let input = EXAMPLE;
    let input = read_input();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert((0, 0));

    let mut knot_positions = vec![(0, 0); 10];

    for line in input.lines() {
        let direction = match line.as_bytes()[0] {
            b'R' => (1, 0),
            b'L' => (-1, 0),
            b'U' => (0, 1),
            b'D' => (0, -1),
            x => panic!("found unexpected direction {x}"),
        };
        let amount: i32 = line[2..].parse().unwrap();
        for _ in 0..amount {
            knot_positions[0] = (
                knot_positions[0].0 + direction.0,
                knot_positions[0].1 + direction.1,
            );
            for i in 1..10 {
                let updated_tail_position =
                    next_tail_position(knot_positions[i - 1], knot_positions[i]);
                knot_positions[i] = updated_tail_position;
            }
            visited_positions.insert(knot_positions[9]);
        }
    }

    println!("{}", visited_positions.len());
}
