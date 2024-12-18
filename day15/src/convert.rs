use std::fs::{self, OpenOptions};

use std::io::Write;

fn convert<'a>(ch: char) -> &'a str {
    match ch {
        '#' => "##",
        '.' => "..",
        'O' => "[]",
        '@' => "@.",
        _ => panic!("Invalid character"),
    }
}

fn main() {
  let args = std::env::args().collect::<Vec<String>>();

    let input = &args[1];
    let data = match fs::read_to_string(input) {
        Ok(data) => data,
        Err(e) => panic!("Could not read file: {}", e),
    };
    let (grid, directions) = data.split_once("\n\n").unwrap();

    let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();

    let width = grid[0].len();
    let height = grid.len();
    let (name, extension) = input.split_once(".").unwrap();
    let output = format!("{}2.{}", name, extension);

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output)
        .unwrap();
    for row in 0..height {
        for col in 0..width {
            write!(f, "{}", convert(grid[row][col])).expect("Could not write to file");
        }
        write!(f, "\n").unwrap();
    }

    let msg = format!("\n\n{}", directions);
    write!(f, "{}", msg).unwrap();
}
