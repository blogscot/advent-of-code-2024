mod point;

use std::collections::{HashSet, VecDeque};

use point::Point;

const WALL: char = '#';
const EMPTY: char = '.';
const ROBOT: char = '@';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';

#[derive(Debug, Clone, Copy)]
struct Move {
    x: i32,
    y: i32,
}

impl Move {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Move> for Point {
    fn from(value: Move) -> Self {
        Self::new(value.x, value.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up((i32, i32)),
    Down((i32, i32)),
    Left((i32, i32)),
    Right((i32, i32)),
}

fn parse_direction(direction: char) -> Direction {
    match direction {
        '^' => Direction::Up((0, -1)),
        'v' => Direction::Down((0, 1)),
        '<' => Direction::Left((-1, 0)),
        '>' => Direction::Right((1, 0)),
        _ => panic!("Invalid direction"),
    }
}

#[derive(Debug)]
struct WareHouse {
    robot: Point,
    grid: Vec<Vec<char>>,
    width: u32,
    height: u32,
}

impl WareHouse {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let mut robot = Point::new(0, 0);
        let width = grid[0].len() as u32;
        let height = grid.len() as u32;

        for row in 0..height {
            for col in 0..width {
                let point = Point::new(col as i32, row as i32);
                if grid[row as usize][col as usize] == ROBOT {
                    robot = point;
                    break;
                }
            }
        }
        Self {
            robot,
            grid,
            width,
            height,
        }
    }
    fn at(&self, point: Point) -> Option<char> {
        if self.is_valid(point) {
            Some(self.grid[point.y as usize][point.x as usize])
        } else {
            None
        }
    }
    fn set(&mut self, point: Point, value: char) {
        self.grid[point.y as usize][point.x as usize] = value;
    }
    fn is_valid(&self, point: Point) -> bool {
        let Point { x, y } = point;
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }
    fn display(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point::new(col as i32, row as i32);
                print!("{}", self.at(point).unwrap());
            }
            println!();
        }
    }

    // Returns the first empty position that a box can be moved left or right
    fn find_empty_position_horizontally(&self, box_point: Point, delta: Move) -> Option<Point> {
        let mut next_point = box_point + delta.into();
        while let Some(object) = self.at(next_point) {
            match object {
                WALL => return None,
                EMPTY => return Some(next_point),
                _ => (),
            }
            next_point = next_point + delta.into();
        }
        None
    }
    fn contains_wall(&self, points: &[Point]) -> bool {
        points.iter().any(|p| self.at(*p) == Some(WALL))
    }
    fn get_move(&self, direction: &Direction) -> Move {
        match direction {
            Direction::Up((x, y)) => Move::new(*x, *y),
            Direction::Down((x, y)) => Move::new(*x, *y),
            Direction::Left((x, y)) => Move::new(*x, *y),
            Direction::Right((x, y)) => Move::new(*x, *y),
        }
    }
    fn move_points(&mut self, points: Vec<Point>) {
        points
            .iter()
            .zip(points.iter().skip(1))
            .for_each(|(p1, p2)| {
                self.set(*p1, self.at(*p2).unwrap());
            });
    }

    fn find_movable_boxes(&self, b: Box, direction: &Direction) -> HashSet<Box> {
        let mut queue: VecDeque<Box> = VecDeque::new();
        let mut moveable = HashSet::new();
        queue.push_back(b);
        while !queue.is_empty() {
            let b = queue.pop_front().unwrap();
            moveable.insert(b);
            let adjacent_points = match *direction {
                Direction::Up((0, -1)) => b.space_above(),
                Direction::Down((0, 1)) => b.space_below(),
                _ => panic!("Invalid direction"),
            };
            if self.contains_wall(&adjacent_points) {
                return HashSet::default();
            }
            adjacent_points.iter().for_each(|point| {
                let ch = self.at(*point).unwrap();
                if ch == BOX_LEFT || ch == BOX_RIGHT {
                    queue.push_back(Box::new(ch, *point));
                }
            })
        }
        moveable
    }

    fn clear_box(&mut self, b: &Box) {
        self.set(b.left, EMPTY);
        self.set(b.right, EMPTY);
    }
    fn set_box(&mut self, b: &Box, direction: &Direction) {
        let delta = self.get_move(direction);
        self.set(b.left + delta.into(), BOX_LEFT);
        self.set(b.right + delta.into(), BOX_RIGHT);
    }

    fn update_robot(&mut self, point: &Point) {
        self.set(self.robot, EMPTY);
        self.set(*point, ROBOT);
        self.robot = *point;
    }

    fn move_robot(&mut self, direction: &Direction) {
        match *direction {
            Direction::Up((0, -1)) | Direction::Down((0, 1)) => {
                self.move_robot_vertically(direction)
            }
            _ => self.move_robot_horizontally(direction),
        }
    }
    fn move_robot_vertically(&mut self, direction: &Direction) {
        let delta = self.get_move(direction);
        let new_point = self.robot + delta.into();
        let ch = self.at(new_point).unwrap();
        match ch {
            WALL => (),
            EMPTY => {
                self.update_robot(&new_point);
            }
            BOX_LEFT | BOX_RIGHT => {
                let b = Box::new(ch, new_point);
                let movable_boxes = self.find_movable_boxes(b, direction);
                movable_boxes.iter().for_each(|b| self.clear_box(b));
                movable_boxes
                    .iter()
                    .for_each(|b| self.set_box(b, direction));
                if !movable_boxes.is_empty() {
                    self.update_robot(&new_point);
                }
            }
            _ => panic!("Invalid character found {}", ch),
        }
    }

    fn move_robot_horizontally(&mut self, direction: &Direction) {
        let robot_point = self.robot;
        let delta = self.get_move(direction);
        let new_point = self.robot + delta.into();
        match self.at(new_point).unwrap() {
            WALL => (),
            EMPTY => {
                self.update_robot(&new_point);
            }
            // if there is a space between the box and the wall you can move the box
            BOX_RIGHT => {
                if let Some(empty_point) = self.find_empty_position_horizontally(new_point, delta) {
                    let points: Vec<Point> = (empty_point.x..=robot_point.x)
                        .map(|x| Point::new(x, robot_point.y))
                        .collect();
                    self.move_points(points);
                    self.update_robot(&new_point);
                }
            }
            BOX_LEFT => {
                if let Some(empty_point) = self.find_empty_position_horizontally(new_point, delta) {
                    let points: Vec<Point> = (robot_point.x..=empty_point.x)
                        .map(|x| Point::new(x, robot_point.y))
                        .rev()
                        .collect();
                    self.move_points(points);
                    self.update_robot(&new_point);
                }
            }
            _ => panic!("Invalid move"),
        }
    }
    fn stocktake(&self) -> u32 {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point::new(col as i32, row as i32);
                if self.at(point) == Some(BOX_LEFT) {
                    sum += row * 100 + col;
                }
            }
        }
        sum
    }
}

fn convert_square<'a>(ch: char) -> &'a str {
    match ch {
        '#' => "##",
        '.' => "..",
        'O' => "[]",
        '@' => "@.",
        _ => panic!("Invalid character"),
    }
}

fn convert(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut converted = Vec::new();
    for row in grid {
        let mut new_row = Vec::new();
        for ch in row {
            let doubled = convert_square(ch);
            new_row.extend(doubled.chars());
        }
        converted.push(new_row);
    }
    converted
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Box {
    left: Point,
    right: Point,
}

impl Box {
    fn new(ch: char, point: Point) -> Self {
        match ch {
            '[' => Self {
                left: point,
                right: Point::new(point.x + 1, point.y),
            },
            ']' => Self {
                left: Point::new(point.x - 1, point.y),
                right: point,
            },
            _ => panic!("Invalid charactier {}", ch),
        }
    }
    fn space_above(&self) -> Vec<Point> {
        let above_left = self.left + Point::new(0, -1);
        let above_right = self.right + Point::new(0, -1);
        vec![above_left, above_right]
    }
    fn space_below(&self) -> Vec<Point> {
        let below_left = self.left + Point::new(0, 1);
        let below_right = self.right + Point::new(0, 1);
        vec![below_left, below_right]
    }
}

fn main() {
    let (grid, directions) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let directions = directions.replace("\n", "");

    let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let mut warehose = WareHouse::new(convert(grid));

    let instructions: Vec<Direction> = directions.chars().map(parse_direction).collect();
    // warehose.display();
    instructions.iter().for_each(|direction| {
        warehose.move_robot(direction);
    });
    warehose.display();

    println!("{}", warehose.stocktake());
}
