use std::{fs::File, io::Read};

use nom::{
    bytes::complete::tag, character::complete::line_ending, multi::separated_list0, IResult,
};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day13.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, items) = parse_list(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Packet { items }))
}

fn parse_list(input: &str) -> IResult<&str, Vec<PacketItem>> {
    let (input, _) = tag("[")(input)?;
    let (input, items) = separated_list0(tag(","), parse_item)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, items))
}

fn parse_item(input: &str) -> IResult<&str, PacketItem> {
    let parse_value = nom::combinator::map(nom::character::complete::u32, PacketItem::Value);
    let parse_list_next = nom::combinator::map(parse_list, PacketItem::List);
    nom::branch::alt((parse_value, parse_list_next))(input)
}

#[derive(Debug, Default, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Packet {
    items: Vec<PacketItem>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketItem {
    List(Vec<PacketItem>),
    Value(u32),
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketItem::Value(self_value), PacketItem::Value(other_value)) => {
                self_value.cmp(other_value)
            }
            (PacketItem::List(self_list), PacketItem::List(other_list)) => {
                self_list.cmp(other_list)
            }
            (PacketItem::Value(self_value), PacketItem::List(other_list)) => {
                let self_list = vec![PacketItem::Value(*self_value)];
                self_list.cmp(other_list)
            }
            (PacketItem::List(self_list), PacketItem::Value(other_value)) => {
                let other_list = vec![PacketItem::Value(*other_value)];
                self_list.cmp(&other_list)
            }
        }
    }
}

#[allow(unused)]
const EXAMPLE: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

pub fn part1() {
    // let mut input = EXAMPLE;
    let base_input = read_input();
    let mut input: &str = &base_input;

    let mut index = 1;
    let mut acc = 0;
    loop {
        let (remaining, p1) = parse_packet(input).unwrap();
        let (remaining, p2) = parse_packet(remaining).unwrap();
        match p1.cmp(&p2) {
            std::cmp::Ordering::Less => {
                // println!("less at index {index}");
                acc += index
            }
            std::cmp::Ordering::Equal => panic!("found equal inputs at index {index}"),
            std::cmp::Ordering::Greater => {
                // println!("greater at index {index}");
            }
        }

        input = remaining;
        if !input.is_empty() {
            input = &input[1..]; // skip empty line
        } else {
            break;
        }
        index += 1;
    }
    println!("{acc}");
}

pub fn part2() {}
