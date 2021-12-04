extern crate advent;

use advent::input_from_args;

#[derive(Debug, Clone, PartialEq)]
struct Movement {
    direction: String,
    distance: i32,
}

fn main() {
    let input = input_from_args();

    let r = input
        .map(|l| {
            let l = l.expect("Could not read line");
            let mut m = l.split_whitespace();

            Movement {
                direction: m.next().unwrap().to_string(),
                distance: m.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Movement>>();

    println!("{}", basic_dive(r.clone()));
    println!("{}", aimed_dive(r.clone()));
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
            match m.direction.as_str() {
                "forward" => {
                    pos.horizontal += m.distance;
                }
                "up" => {
                    pos.depth -= m.distance;
                }
                "down" => {
                    pos.depth += m.distance;
                }
                _ => panic!("Unknown direction {}", m.direction),
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
            match m.direction.as_str() {
                "forward" => {
                    pos.horizontal += m.distance;
                    pos.depth += pos.aim * m.distance
                }
                "up" => {
                    pos.aim -= m.distance;
                }
                "down" => {
                    pos.aim += m.distance;
                }
                _ => panic!("Unknown direction: {}", m.direction),
            }

            pos
        },
    );

    out.horizontal * out.depth
}
