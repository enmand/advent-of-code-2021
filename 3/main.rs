extern crate advent;

use ndarray::{iter::Lanes, Array2, ArrayBase, Axis, Dim};

use advent::input_from_args;

#[derive(Debug, Clone, Copy)]
struct Counts {
    zeros: u32,
    ones: u32,
}

fn main() {
    let lines = input_from_args().peekable();

    let bin = lines
        .map(|l| {
            l.expect("unable to read line")
                .chars()
                .map(|c| {
                    i32::from_str_radix(c.encode_utf8(&mut [c as u8]).as_ref(), 2)
                        .expect("unable to parse debug digit")
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let raw: Vec<Vec<i32>> = bin.clone();
    let data = Array2::from_shape_vec(
        (raw.len(), raw[0].len()),
        raw.into_iter().flatten().collect(),
    )
    .expect("load ndarray");

    let lanes = data.lanes(Axis(0));
    let count = counts(lanes);

    println!("{}", power(count.clone()));
    println!("{}", life_support(count, bin));
}

fn power(count: Vec<Counts>) -> i32 {
    power_value(count.clone(), 1, 0) * power_value(count.clone(), 0, 1)
}

fn power_value(debug: Vec<Counts>, high: i32, low: i32) -> i32 {
    i32::from_str_radix(
        debug
            .clone()
            .into_iter()
            .map(|c| {
                if c.ones > c.zeros {
                    high.to_string()
                } else {
                    low.to_string()
                }
            })
            .reduce(|acc, s| format!("{}{}", acc, s))
            .expect("no value")
            .as_str(),
        2,
    )
    .expect("no value")
}

fn life_support(counts: Vec<Counts>, debug: Vec<Vec<i32>>) -> i32 {
    let o = support(counts.clone(), debug.clone(), 1, 0);
    let co2_scrub = support(counts.clone(), debug, 0, 1);

    o * co2_scrub
}

fn support(cs: Vec<Counts>, mut lines: Vec<Vec<i32>>, high: i32, low: i32) -> i32 {
    let mut offset = 0;
    for i in 0..cs.len() {
        let raw = lines.clone();
        let data = Array2::from_shape_vec(
            (raw.len(), raw[0].len()),
            raw.into_iter().flatten().collect(),
        )
        .expect("parsing filtered count");
        let lanes = data.lanes(Axis(0));
        let c = &counts(lanes)[i];

        if c.ones > c.zeros || c.ones == c.zeros {
            lines = lines.into_iter().filter(|l| l[offset] == high).collect();
        } else {
            lines = lines.into_iter().filter(|l| l[offset] == low).collect();
        }

        if lines.len() == 1 {
            break;
        }

        offset = offset + 1;
    }

    return i32::from_str_radix(
        &lines
            .get(0)
            .expect("unable to find value")
            .into_iter()
            .map(|b| b.to_string())
            .reduce(|acc, s| format!("{}{}", acc, s))
            .expect("determing final value"),
        2,
    )
    .expect("no value");
}

fn counts(lanes: Lanes<i32, Dim<[usize; 1]>>) -> Vec<Counts> {
    lanes
        .into_iter()
        .map(|l| {
            let mut counts = Counts { zeros: 0, ones: 0 };
            for d in l.iter() {
                if *d == 0 {
                    counts.zeros += 1;
                } else {
                    counts.ones += 1;
                }
            }
            counts
        })
        .collect::<Vec<Counts>>()
}
