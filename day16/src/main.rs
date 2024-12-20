mod point;

use phf::phf_map;

use priority_queue::DoublePriorityQueue;
use std::collections::{HashMap, HashSet};

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
    fn is_valid(&self, point: &Point) -> bool {
        self.at(point) != WALL
    }
    fn get_directions(&self, current: Point) -> Vec<(&char, Point)> {
        DIRECTIONS
            .entries()
            .map(|(direction, &point)| (direction, current + point.into()))
            .filter(|(_, point)| self.at(point) != WALL)
            .collect()
    }
    fn navigate(&self) -> usize {
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
            for (direction, neighbor) in self.get_directions(*last) {
                let mut new_path = current_path.clone();
                if let Some(rotations) = ROTATIONS.get(&current_direction) {
                    new_path.push(neighbor);
                    if rotations.contains(direction) {
                        queue.push((*direction, new_path), cost + 1001);
                    } else {
                        queue.push((*direction, new_path), cost + 1);
                    }
                }
            }
        }
        0
    }

    fn count_tiles(&self, max_cost: i32) -> usize {
        let mut tiles: HashSet<Point> = HashSet::new();
        let mut queue: DoublePriorityQueue<(char, Point, Vec<Point>), i32> = DoublePriorityQueue::new();
        let mut state: HashMap<(char, Point), i32> = HashMap::new();
        queue.push(('E', self.start, vec![self.start]), 0);
        while !queue.is_empty() {
            let ((direction, last, path), cost) = queue.pop_min().unwrap();
            if cost > max_cost {
                continue;
            }
            let entry = (direction, last);
            match state.get(&entry) {
                Some(&old_cost) => {
                    if cost > old_cost {
                        continue;
                    }
                }
                None => {
                    state.insert(entry, cost);
                }
            }

            if last == self.end && cost == max_cost {
                for point in &path {
                    tiles.insert(*point);
                }
                continue;
            }

            let delta = *DIRECTIONS.get(&direction).unwrap();
            let new_point = last + delta.into();
            if self.is_valid(&new_point) {
                let mut new_path = path.clone();
                new_path.push(new_point);
                let new_cost = cost + 1;
                if new_cost <= max_cost {
                    queue.push((direction, new_point, new_path), new_cost);
                }
            }

            for new_direction in *ROTATIONS.get(&direction).unwrap() {
                let new_cost = cost + 1000;
                if new_cost <= max_cost {
                    queue.push((new_direction, last, path.clone()), new_cost);
                }
            }
        }
        tiles.len()
    }
}

fn solve_part_1(maze: &Maze) -> usize {
    maze.navigate()
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("puzzle.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let maze = Maze::new(input);
    let cost = solve_part_1(&maze);
    println!("Part 1: {:?}", cost);
    println!("Part 2: {:?}", maze.count_tiles(cost as i32));
}
