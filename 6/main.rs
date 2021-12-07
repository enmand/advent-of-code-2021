extern crate advent;

use std::ops::Range;

use advent::input_from_args;

fn main() {
    let input = input_from_args()
        .map(|l| l.expect("no line found"))
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .next()
        .expect("unable to read input");

    println!("{:?}", solve(input.clone(), 0..80));
    println!("{:?}", solve(input, 0..256));
}

fn solve(fish: Vec<i32>, days: Range<i32>) -> i128 {
    let mut timings: Vec<i128> = (0..9).into_iter().map(|_| 0).collect();

    fish.iter().for_each(|&t| timings[t as usize] += 1);

    for _ in days {
        let incoming = timings[0];

        timings.clone().iter().enumerate().for_each(|(k, &t)| {
            if k > 0 {
                timings[k - 1] = t;
            }
        });

        if incoming > 0 {
            timings[6] += incoming;
        }
        timings[8] = incoming;
    }

    timings.iter().sum::<i128>()
}
