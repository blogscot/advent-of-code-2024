mod point;

use phf::phf_map;

use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;

use point::Point;

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';

type Direction = phf::Map<char, (i32, i32)>;
type Rotation = phf::Map<char, [char; 2]>;

const DIRECTIONS: Direction = {
    phf_map! {
        'N' => (0, -1),
        'S' => (0, 1),
        'E' => (1, 0),
        'W' => (-1, 0),
    }
};

const ROTATIONS: Rotation = {
    phf_map! {
        'N' => ['W', 'E'],
        'S' => ['E', 'W'],
        'E' => ['N', 'S'],
        'W' => ['S', 'N'],
    }
};

#[derive(Debug, Clone)]
struct Maze {
    maze: Vec<Vec<char>>,
    start: Point,
    end: Point,
}

fn find_start_end(maze: &[Vec<char>]) -> Option<(Point, Point)> {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x] == END {
                end = Some(Point::new(x as i32, y as i32));
                break;
            }
            if maze[y][x] == START {
                start = Some(Point::new(x as i32, y as i32));
                break;
            }
        }
    }
    match (start, end) {
        (Some(start), Some(end)) => Some((start, end)),
        _ => None,
    }
}

impl Maze {
    fn new(maze: Vec<Vec<char>>) -> Self {
        let (start, end) = find_start_end(&maze).unwrap();
        Self { maze, start, end }
    }
    fn at(&self, point: &Point) -> char {
        self.maze[point.y as usize][point.x as usize]
    }
    fn set(&mut self, point: &Point, ch: char) {
        self.maze[point.y as usize][point.x as usize] = ch;
    }
    fn get_directions(&self, current: Point) -> Vec<(char, Point)> {
        DIRECTIONS
            .keys()
            .map(|&direction| {
                let (x, y) = DIRECTIONS.get(&direction).unwrap();
                (direction, current + Point::new(*x, *y))
            })
            .filter(|(_, point)| self.at(point) != WALL)
            .collect()
    }
    fn navigate(&self) -> usize {
        let mut paths = vec![];
        let mut queue: DoublePriorityQueue<(char, Vec<Point>), i32> = DoublePriorityQueue::new();
        let mut visited: HashSet<Point> = HashSet::new();
        queue.push(('E', vec![self.start]), 0);
        while !queue.is_empty() {
            let ((current_direction, current_path), cost) = queue.pop_min().unwrap();
            let last = current_path.last().unwrap();
            if self.at(last) == END {
                return cost as usize;
            }
            if visited.contains(last) {
                continue;
            }
            visited.insert(*last);
            if *last == self.end {
                paths.push(current_path);
                continue;
            }
            for (direction, neighbor) in self.get_directions(*last) {
                let mut new_path = current_path.clone();
                if let Some(rotations) = ROTATIONS.get(&current_direction) {
                    new_path.push(neighbor);
                    if rotations.contains(&direction) {
                        queue.push((direction, new_path.clone()), cost + 1001);
                    } else {
                        queue.push((direction, new_path), cost + 1);
                    }
                }
            }
        }
        0
    }
    fn display(&self) {
        for row in self.maze.iter() {
            for ch in row.iter() {
                print!("{}", ch);
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("puzzle.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let maze = Maze::new(input);
    let costs = maze.navigate();

    println!("{:?}", costs);
}
