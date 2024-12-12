mod point;

use std::collections::HashSet;

use point::Point;

type Direction = (i32, i32);
type Path = Vec<Point>;

const UP: Direction = (-1, 0);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const RIGHT: Direction = (0, 1);

const DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

struct Topograph {
    grid: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Topograph {
    fn new(grid: Vec<Vec<u32>>) -> Topograph {
        Topograph {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    fn at(&self, point: &Point) -> Option<u32> {
        if self.is_valid(point) {
            Some(self.grid[point.row as usize][point.col as usize])
        } else {
            None
        }
    }

    fn is_valid(&self, point: &Point) -> bool {
        let Point { row, col } = *point;
        row >= 0 && col >= 0 && (row as usize) < self.height && (col as usize) < self.width
    }

    fn find_trailheads(&self) -> Vec<Point> {
        (0..self.height)
            .flat_map(|row| {
                (0..self.width)
                    .map(move |col| Point::new(row as i32, col as i32))
                    .filter(|point| self.at(point) == Some(0))
            })
            .collect()
    }

    fn find_walked_trails(&self, trailhead: Point) -> HashSet<Path> {
        let mut paths: Vec<Path> = Vec::new();
        let mut walked: HashSet<Path> = HashSet::new();
        paths.push(vec![trailhead]);

        while let Some(path) = paths.pop() {
            let last = path.last().unwrap();
            let neighbours = self.find_next(*last);

            for neighbour in neighbours {
                let mut new_path = path.clone();
                new_path.push(neighbour);
                if self.reached_summit(&new_path) {
                    walked.insert(new_path);
                } else {
                    paths.push(new_path);
                }
            }
        }
        walked
    }

    // Finds the next points on the trail if any
    fn find_next(&self, point: Point) -> Vec<Point> {
        let current = self.at(&point).unwrap();
        let mut output = Vec::new();

        DIRECTIONS.iter().for_each(|(row, col)| {
            let neighbour = point + Point::new(*row, *col);
            if let Some(val) = self.at(&neighbour) {
                if val == current + 1 {
                    output.push(neighbour);
                }
            }
        });
        output
    }

    fn reached_summit(&self, path: &Path) -> bool {
        let last = path.last().unwrap();
        self.at(last) == Some(9)
    }
}

fn solve_part1(topo: &Topograph) -> usize {
    topo.find_trailheads()
        .into_iter()
        .map(|trailhead| {
            topo.find_walked_trails(trailhead)
                .into_iter()
                .map(|path| *path.last().unwrap())
                .collect::<HashSet<Point>>()
                .len()
        })
        .sum()
}

fn solve_part2(topo: &Topograph) -> usize {
    topo.find_trailheads()
        .into_iter()
        .map(|trailhead| topo.find_walked_trails(trailhead).len())
        .sum()
}

fn main() {
    let grid: Vec<Vec<u32>> = include_str!("puzzle.txt")
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let topo = Topograph::new(grid);

    println!("Part 1: {:?}", solve_part1(&topo));
    println!("Part 2: {:?}", solve_part2(&topo));
}
