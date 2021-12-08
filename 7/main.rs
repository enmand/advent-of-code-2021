extern crate advent;

fn main() {
    let input = advent::input_from_args()
        .map(|l| l.expect("could not read line"))
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<i128>().unwrap())
                .collect::<Vec<_>>()
        })
        .next()
        .expect("no input");

    println!("{:?}", solve(&input, |x| x));
    println!("{:?}", solve(&input, |x| (x * (x + 1) / 2)));
}

fn solve<F>(input: &Vec<i128>, f: F) -> i128
where
    F: Fn(i128) -> i128,
{
    (0..input
        .clone()
        .into_iter()
        .max()
        .expect("unable to determine column count"))
        .map(|v| input.clone().into_iter().map(|n| f((n - v).abs())).sum())
        .min()
        .expect("unable to determine minimum")
}

#[test]
fn example() {
    assert_eq!(solve(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], |x| x), 37);
}

#[test]
fn example_2() {
    assert_eq!(
        solve(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], |x| ((x * (x + 1))
            / 2)),
        168
    );
}
