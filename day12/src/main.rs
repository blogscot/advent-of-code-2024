mod point;

use std::collections::HashSet;

use point::Point;

type Direction = (i32, i32);

const UP: Direction = (-1, 0);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const RIGHT: Direction = (0, 1);

const DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Debug)]
struct Plot {
    points: HashSet<Point>,
    perimeter: u32,
}

#[derive(Debug)]
struct Garden {
    garden: Vec<Vec<char>>,
    uncharted: HashSet<Point>,
    width: i32,
    height: i32,
}

impl Garden {
    fn new(garden: Vec<Vec<char>>) -> Self {
        let width = garden[0].len() as i32;
        let height = garden.len() as i32;
        let uncharted = (0..width)
            .flat_map(|row| (0..height).map(move |col| Point::new(row, col)))
            .collect();
        Self {
            width,
            height,
            garden,
            uncharted,
        }
    }

    fn mark_charted(&mut self, plot: &Plot) {
        for point in plot.points.iter() {
            self.uncharted.remove(point);
        }
    }

    fn is_valid(&self, point: Point) -> bool {
        let Point { row, col } = point;
        row >= 0 && row < self.height && col >= 0 && col < self.width
    }

    fn at(&self, point: Point) -> char {
        self.garden[point.row as usize][point.col as usize]
    }

    fn find_neighbours(&self, point: Point) -> Vec<Point> {
        let label = self.garden[point.row as usize][point.col as usize];
        DIRECTIONS
            .iter()
            .map(|(row, col)| point + Point::new(*row, *col))
            .filter(|p| self.is_valid(*p) && self.at(*p) == label)
            .collect()
    }
    fn build_plot(&self, initial: Point) -> Plot {
        let mut queue = vec![initial];
        let mut found = vec![initial];
        while let Some(point) = queue.pop() {
            let neighbours = self.find_neighbours(point);
            neighbours.iter().for_each(|p| {
                if !found.contains(p) {
                    found.push(*p);
                    queue.push(*p);
                }
            });
        }
        let perimeter = self.calculate_perimeter(&found);
        Plot {
            points: HashSet::from_iter(found),
            perimeter,
        }
    }

    fn calculate_perimeter(&self, points: &[Point]) -> u32 {
        points.iter().map(|p| {
            let neighbours = self.find_neighbours(*p);
            4 - neighbours.len() as u32
        }).sum()
    }
}

fn solve_part1(grid: &[Vec<char>]) -> u32 {
    let mut garden = Garden::new(grid.to_vec());
    let mut sum = 0;
    while let Some(point) = garden.uncharted.iter().next() {
        let plot = garden.build_plot(*point);
        garden.mark_charted(&plot);
        sum += plot.points.len() as u32 * plot.perimeter
    }
    sum
}

fn main() {
    let grid: Vec<Vec<char>> = include_str!("puzzle.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 1: {}", solve_part1(&grid));
}
