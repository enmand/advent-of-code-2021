use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;

pub fn input_from_args() -> std::io::Lines<BufReader<File>> {
    let args = env::args().collect::<Vec<String>>();
    let input = args_file(args, 1);

    let f = File::open(input).expect("file not found");
    BufReader::new(f).lines()
}

pub fn args_file(args: Vec<String>, skip: usize) -> String {
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
