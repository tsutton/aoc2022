use std::{collections::HashMap, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day6.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[allow(unused)]
const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

pub fn part1() {
    // let input = EXAMPLE;
    let input = read_input();
    for (i, window) in input.as_bytes().windows(4).enumerate() {
        if window[0] == window[1]
            || window[0] == window[2]
            || window[0] == window[3]
            || window[1] == window[2]
            || window[1] == window[3]
            || window[2] == window[3]
        {
            continue;
        } else {
            println!("{}", i + 4);
            return;
        }
    }
}

pub fn part2() {
    // let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let input = read_input();
    let mut char_set = HashMap::<u8, usize>::new();

    // for each loop iteration, at the end of the iteration, this map has the count of the most recent 13 chars
    // then the next iteratoin can add the 14th, check, and decrement the count / possibly remove the first.

    for (i, window) in input.as_bytes().windows(14).enumerate() {
        if i == 0 {
            for c in &window[1..] {
                *char_set.entry(*c).or_insert(0) += 1;
            }
            // assume the first 14 chars aren't all different
            continue;
        }
        // println!("iteration {i}: {char_set:?}");
        assert!(char_set.values().sum::<usize>() == 13);
        char_set
            .entry(window[13])
            .and_modify(|c| *c += 1)
            .or_insert(1);
        if char_set.len() == 14 {
            println!("{}", i + 14);
            return;
        } else {
            char_set.entry(window[0]).and_modify(|c| *c -= 1);
            if char_set[&window[0]] == 0 {
                char_set.remove(&window[0]);
            }
        }
    }
}
