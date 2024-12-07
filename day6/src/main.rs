use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}

const OBSTACLE: char = '#';

type Direction = (isize, isize);

const UP: Direction = (-1, 0);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const RIGHT: Direction = (0, 1);

struct Grid {
    grid: Vec<Vec<char>>,
    position: Position,
    direction: Direction,
    width: usize,
    height: usize,
    visited: HashSet<Position>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        let guard = (0..height)
            .flat_map(|row| (0..width).map(move |col| (row, col)))
            .find(|(row, col)| grid[*row][*col] == '^')
            .unwrap();
        let mut visited = HashSet::new();
        visited.insert(Position::new(guard.0, guard.1));
        Self {
            grid,
            position: Position::new(guard.0, guard.1),
            direction: UP,
            width,
            height,
            visited,
        }
    }

    fn turn(&mut self) {
        match self.direction {
            UP => self.direction = RIGHT,
            DOWN => self.direction = LEFT,
            LEFT => self.direction = UP,
            RIGHT => self.direction = DOWN,
            _ => panic!("Unknown direction"),
        }
    }

    fn found_exit(&self) -> bool {
        let Position { row, col } = self.position;
        if row == 0 || col == 0 || row == self.height - 1 || col == self.width - 1 {
            return true;
        }
        false
    }

    fn step(&mut self) {
        let Position { row, col } = self.position;
        let (drow, dcol) = self.direction;
        let new_row = row as isize + drow;
        let new_col = col as isize + dcol;
        if self.grid[new_row as usize][new_col as usize] == OBSTACLE {
            self.turn();
            self.step();
        } else {
            let position = Position::new(new_row as usize, new_col as usize);
            self.position = position.clone();
            self.visited.insert(position);
        }
    }
}

fn solve_part_1(grid: &mut Grid) -> usize {
    while !grid.found_exit() {
        grid.step();
    }
    grid.visited.len()
}

struct Grid2 {
    grid: Vec<Vec<char>>,
    initial_position: Position,
    position: Position,
    direction: Direction,
    width: usize,
    height: usize,
    visited: HashSet<Position>,
    new_obstacle: Option<Position>,
    collisions: HashMap<Position, HashSet<Direction>>,
    cycle_detected: bool,
}

impl Grid2 {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        let guard = (0..height)
            .flat_map(|row| (0..width).map(move |col| (row, col)))
            .find(|(row, col)| grid[*row][*col] == '^')
            .unwrap();
        let mut visited = HashSet::new();
        visited.insert(Position::new(guard.0, guard.1));
        Self {
            grid,
            initial_position: Position::new(guard.0, guard.1),
            position: Position::new(guard.0, guard.1),
            direction: UP,
            width,
            height,
            visited,
            new_obstacle: None,
            collisions: HashMap::new(),
            cycle_detected: false,
        }
    }

    fn reset(&mut self) {
        self.position = self.initial_position.clone();
        self.direction = UP;
        self.collisions.clear();
        self.clear_obstacle();
        self.cycle_detected = false;
    }

    fn turn(&mut self) {
        match self.direction {
            UP => self.direction = RIGHT,
            DOWN => self.direction = LEFT,
            LEFT => self.direction = UP,
            RIGHT => self.direction = DOWN,
            _ => panic!("Unknown direction"),
        }
    }

    fn found_exit(&self) -> bool {
        let Position { row, col } = self.position;
        row == 0 || col == 0 || row == self.height - 1 || col == self.width - 1
    }

    fn set_new_obstacle(&mut self, position: &Position) {
        self.new_obstacle = Some(position.clone());
        self.grid[position.row][position.col] = OBSTACLE;
    }

    fn clear_obstacle(&mut self) {
        if let Some(position) = &self.new_obstacle {
            self.grid[position.row][position.col] = '.';
        }
    }

    fn detect_cycle(&mut self, position: &Position) {
        match self.collisions.get(position) {
            Some(directions) => {
                if directions.contains(&self.direction) {
                    self.cycle_detected = true;
                } else {
                    self.collisions
                        .get_mut(position)
                        .unwrap()
                        .insert(self.direction);
                }
            }
            None => {
                let directions = HashSet::from([self.direction]);
                self.collisions.insert(position.clone(), directions);
            }
        }
    }

    fn step(&mut self) {
        let Position { row, col } = self.position;
        let (drow, dcol) = self.direction;
        let new_row = row as isize + drow;
        let new_col = col as isize + dcol;
        let position = Position::new(new_row as usize, new_col as usize);
        if self.grid[new_row as usize][new_col as usize] == OBSTACLE {
            self.detect_cycle(&position);
            self.turn();
            self.step();
        } else {
            let position = Position::new(new_row as usize, new_col as usize);
            self.position = position.clone();
            self.visited.insert(position);
        }
    }
}

fn solve_part2(grid: Vec<Vec<char>>, visited: &HashSet<Position>) -> usize {
    let mut grid = Grid2::new(grid);
    let mut count = 0;

    visited.iter().for_each(|position| {
        grid.reset();
        grid.set_new_obstacle(position);
        while !grid.found_exit() && !grid.cycle_detected {
            grid.step();
        }
        if grid.cycle_detected {
            count += 1;
        }
    });
    count
}

fn main() {
    let grid: Vec<Vec<char>> = include_str!("puzzle.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut grid1 = Grid::new(grid.clone());
    println!("Part 1: {}", solve_part_1(&mut grid1));

    let visited = grid1.visited.clone();
    println!("Part 2: {}", solve_part2(grid, &visited));
}
