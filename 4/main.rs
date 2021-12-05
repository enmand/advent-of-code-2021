extern crate advent;

use itertools::Itertools;

use advent::input_from_args;
use ndarray::{Array2, ArrayBase, Dim, ViewRepr};

fn main() {
    let mut input = input_from_args();
    let draws = input
        .next()
        .expect("unable to read bingo draw")
        .expect("no input found")
        .split(",")
        .map(|d| d.parse().expect("parse draw"))
        .collect::<Vec<i32>>();

    let boards = input
        .chunks(6)
        .into_iter()
        .map(|c| {
            let s = c
                .skip(1)
                .map(|l| {
                    l.expect("unable to read line")
                        .split_whitespace()
                        .map(|s| {
                            (
                                s.parse().expect(format!("parse line: {}", s).as_str()),
                                false,
                            )
                        })
                        .collect::<Vec<(i32, bool)>>()
                })
                .collect::<Vec<_>>();

            (
                Array2::from_shape_vec((s.len(), s[0].len()), s.into_iter().flatten().collect())
                    .expect("load ndarray"),
                false,
            )
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve(&draws, &boards));
}

fn solve(draws: &[i32], boards: &[(Array2<(i32, bool)>, bool)]) -> (i32, i32) {
    let mut win = 0;
    let mut loss = 0;
    let mut checks: Vec<(Array2<(i32, bool)>, bool)> = boards.clone().to_vec();

    for draw in draws {
        checks = checks
            .into_iter()
            .map(|(mut board, _)| {
                for (i, row) in board.clone().outer_iter().enumerate() {
                    for (j, &(n, _)) in row.iter().enumerate() {
                        if n == *draw {
                            board[(i, j)] = (n, true);
                        }
                    }
                }
                (board.clone(), check(board.clone()))
            })
            .collect::<Vec<_>>();

        let mut w = checks.clone().into_iter().filter(|(_, c)| *c);
        if w.clone().count() == 1 && win == 0 {
            let sum = sum_board(w.next().expect("no winner").0);
            win = sum * draw;
        } else if w.clone().count() == 1 {
            let sum = sum_board(w.next().expect("no loser").0);
            loss = sum * draw;
        }

        checks = checks.into_iter().filter(|(_, c)| !*c).collect();
    }

    (win, loss)
}

fn check(board: Array2<(i32, bool)>) -> bool {
    let rows = board
        .outer_iter()
        .map(|row| check_row(row))
        .collect::<Vec<_>>()
        .iter()
        .any(|&r| r);

    let cols = board
        .columns()
        .into_iter()
        .map(|col| check_col(col))
        .collect::<Vec<_>>()
        .iter()
        .any(|&c| c);

    rows || cols
}

fn check_row(row: ArrayBase<ViewRepr<&(i32, bool)>, Dim<[usize; 1]>>) -> bool {
    row.iter().all(|&(_, b)| b)
}

fn check_col(col: ArrayBase<ViewRepr<&(i32, bool)>, Dim<[usize; 1]>>) -> bool {
    col.iter().all(|&(_, b)| b)
}

fn sum_board(board: Array2<(i32, bool)>) -> i32 {
    board.iter().map(|&(n, m)| if m { 0 } else { n }).sum()
}
