extern crate advent;

use std::collections::HashMap;

use itertools::Itertools;

use advent::input_from_args;

type Point = (i32, i32);
type Line = (Point, Point);

trait PointExt {
    fn points(&self) -> Vec<Point>;
}

impl PointExt for Line {
    fn points(&self) -> Vec<Point> {
        let ((x1, y1), (x2, y2)) = self;
        let mut ps: Vec<Point> = Vec::new();

        if x1 == x2 {
            for y in *y1.min(y2)..=*y2.max(y1) {
                ps.push((*x1, y));
            }
        } else {
            let m = slope(self) as i32; // slope is only 0, 1, -1 or inf so it's safe to cast
            let b = y1 - m * x1;

            let x = *x1.min(x2)..=*x2.max(x1);
            let y = x.clone().map(|x| m * x + b);
            ps.extend(x.zip(y));
        }
        ps
    }
}

fn main() {
    let input = input_from_args();

    let lines = input
        .map(|l| l.expect("Could not read line"))
        .map(|l| {
            let v = l.split("->");
            v.map(|s| {
                let mut d = s.split(",").map(|d| d.trim().to_string()).map(|d| {
                    d.parse::<i32>()
                        .expect(format!("parsing int {}", d).as_str())
                });
                (
                    d.next().expect("cannot find x"),
                    d.next().expect("cannot find y"),
                )
            })
            .collect_tuple()
            .expect("cannot find tuple")
        })
        .collect::<Vec<Line>>();

    println!("{:?}", count_intersections(horizontal_lines(&lines)));
    println!("{:?}", count_intersections(all_lines(&lines)));
}

fn horizontal_lines(lines: &Vec<Line>) -> Vec<Point> {
    lines
        .into_iter()
        .filter(|&l| slope(l) == 0.0 || slope(l) == std::f64::INFINITY)
        .fold(Vec::new(), |mut acc: Vec<Point>, l: &Line| {
            let p = l.points();
            acc.extend(p);
            acc
        })
        .into_iter()
        .collect::<Vec<Point>>()
}

fn all_lines(lines: &Vec<Line>) -> Vec<Point> {
    lines
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<Point>, l: &Line| {
            let p = l.points();
            acc.extend(p);
            acc
        })
        .into_iter()
        .collect::<Vec<Point>>()
}

fn count_intersections(points: Vec<Point>) -> usize {
    points
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<Point, i32>, p: Point| {
            *acc.entry(p).or_insert(0) += 1;
            acc
        })
        .values()
        .filter(|&x| *x >= 2)
        .count()
}

fn slope(((x1, y1), (x2, y2)): &((i32, i32), (i32, i32))) -> f64 {
    let div = x2 - x1;
    if div == 0 {
        f64::INFINITY
    } else {
        (y2 - y1) as f64 / div as f64
    }
}
