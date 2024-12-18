mod point;

use point::Point;

const WALL: char = '#';
const EMPTY: char = '.';
const ROBOT: char = '@';
const BOX: char = 'O';

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

#[derive(Debug)]
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
    fn swap(&mut self, point1: Point, point2: Point) {
        let tmp = self.at(point1).unwrap();
        self.set(point1, self.at(point2).unwrap());
        self.set(point2, tmp);
    }
    // Returns the first valid position that a box can be moved
    fn move_box(&self, box_point: Point, delta: Move) -> Option<Point> {
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
    fn move_robot(&mut self, direction: &Direction) {
        let delta = match direction {
            Direction::Up((x, y)) => Move::new(*x, *y),
            Direction::Down((x, y)) => Move::new(*x, *y),
            Direction::Left((x, y)) => Move::new(*x, *y),
            Direction::Right((x, y)) => Move::new(*x, *y),
        };
        let new_point = self.robot + delta.into();
        match self.at(new_point).unwrap() {
            WALL => (),
            EMPTY => {
                self.set(self.robot, EMPTY);
                self.set(new_point, ROBOT);
                self.robot = new_point;
            }
            BOX => {
                // if there is a space between the box and the wall you can move the box
                // swap the box with the empty space and swap the robot with the box
                if let Some(next_box_point) = self.move_box(new_point, delta) {
                    self.swap(new_point, next_box_point);
                    self.swap(self.robot, new_point);
                    self.robot = new_point;
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
                if self.at(point) == Some(BOX) {
                    sum += row * 100 + col;
                }
            }
        }
        sum
    }
}


fn main() {
    let (grid, directions) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let directions = directions.replace("\n", "");

    let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let mut warehose = WareHouse::new(grid);
    let instructions: Vec<Direction> = directions.chars().map(parse_direction).collect();

    // warehose.display();
    instructions.iter().for_each(|direction| {
        warehose.move_robot(direction);
    });
    warehose.display();

    println!("{}", warehose.stocktake());

}
