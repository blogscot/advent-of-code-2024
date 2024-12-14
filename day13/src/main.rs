use regex::Regex;
extern crate nalgebra as na;
use na::{Matrix2, Vector2};

fn solve_matrix(a: Matrix2<f64>, b: Vector2<f64>) -> Option<Vector2<f64>> {
    let decomp = a.lu();
    decomp.solve(&b)
}

fn approx(val: f64) -> bool {
    (val - val.round()).abs() * 100.0 < 1.0
}

fn solve(data : &Vec<&str>, offset : u64) -> u64 {
    let re = Regex::new(r"X\+(?<x1>\d+), Y\+(?<y1>\d+).* X\+(?<x2>\d+), Y\+(?<y2>\d+).* X=(?<X>\d+), Y=(?<Y>\d+)$").unwrap();

    let mut sum = 0;
    for entry in data {
        let entry =entry.replace("\n", " ");
        let caps = re.captures(&entry).unwrap();
        let x1 = caps["x1"].parse::<u64>().unwrap();
        let y1 = caps["y1"].parse::<u64>().unwrap();
        let x2 = caps["x2"].parse::<u64>().unwrap();
        let y2 = caps["y2"].parse::<u64>().unwrap();
        let x = caps["X"].parse::<u64>().unwrap() + offset;
        let y = caps["Y"].parse::<u64>().unwrap() + offset;
        let a1: Matrix2<f64> = Matrix2::new(x1 as f64, x2 as f64, y1 as f64, y2 as f64);
        let b = Vector2::new(x as f64, y as f64);
        sum += match solve_matrix(a1, b) {
            Some(x) => {
                if approx(x[0]) && approx(x[1]) {
                    x[0].round() as u64 * 3 + x[1].round() as u64
                } else {
                    0
                }
            }
            None => 0,
        }
    }
    sum
}

fn main() {

    let data: Vec<&str> = include_str!("puzzle.txt").split("\n\n").collect();

    println!("Part 1: {}", solve(&data, 0));
    println!("Part 2: {}", solve(&data, 10000000000000));
}
