use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;

const WINDOW_SIZE: usize = 3;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args_file(args, 1);

    let f = File::open(input).expect("file not found");
    let r = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("single increases {}", point_increasese(r.clone()));
    println!("slide increases {}", slide_increasese(r.clone()));
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

fn point_increasese(r: Vec<i32>) -> i32 {
    increasing(r)
}

fn slide_increasese(r: Vec<i32>) -> i32 {
    increasing(
        r.iter()
            .enumerate()
            .map(|(i, x)| {
                if i + WINDOW_SIZE > r.len() {
                    return 0;
                }

                let next = r[i + 1];
                let next_next = r[i + 2];

                next + next_next + *x
            })
            .collect(),
    )
}

fn increasing(r: Vec<i32>) -> i32 {
    r.iter()
        .enumerate()
        .map(|(i, x)| {
            if i == 0 {
                return 0;
            }

            let prev = r[i - 1];

            if prev < *x {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<i32>()
}
