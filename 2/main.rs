use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Forward, Direction::Forward) => true,
            (Direction::Down, Direction::Down) => true,
            (Direction::Up, Direction::Up) => true,
            (Direction::Forward, Direction::Up) => false,
            (Direction::Forward, Direction::Down) => false,
            (Direction::Up, Direction::Forward) => false,
            (Direction::Up, Direction::Down) => false,
            (Direction::Down, Direction::Forward) => false,
            (Direction::Down, Direction::Up) => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug, Clone)]
struct Movement {
    direction: Direction,
    distance: i32,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args_file(args, 1);

    let f = File::open(input).expect("file not found");
    let r = BufReader::new(f)
        .lines()
        .map(|l| {
            let l = l.expect("Could not read line");
            let mut m = l.split_whitespace();

            Movement {
                direction: m.next().unwrap().parse::<Direction>().unwrap(),
                distance: m.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Movement>>();

    println!("{}", basic_dive(r.clone()));
    println!("{}", aimed_dive(r.clone()));
}

fn args_file(args: Vec<String>, skip: usize) -> String {
    let input = args
        .iter()
        .skip(skip)
        .next()
        .expect("No input file provided")
        .deref();

    if input == "." {
        return args_file(args, 2);
    } else {
        return input.into();
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn basic_dive(r: Vec<Movement>) -> i32 {
    let out = r.iter().fold(
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |mut pos, m| {
            match m.direction {
                Direction::Forward => {
                    pos.horizontal += m.distance;
                }
                Direction::Up => {
                    pos.depth -= m.distance;
                }
                Direction::Down => {
                    pos.depth += m.distance;
                }
            }

            pos
        },
    );

    out.horizontal * out.depth
}

fn aimed_dive(r: Vec<Movement>) -> i32 {
    let out = r.iter().fold(
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |mut pos, m| {
            match m.direction {
                Direction::Forward => {
                    pos.horizontal += m.distance;
                    pos.depth += pos.aim * m.distance
                }
                Direction::Up => {
                    pos.aim -= m.distance;
                }
                Direction::Down => {
                    pos.aim += m.distance;
                }
            }

            pos
        },
    );

    out.horizontal * out.depth
}
