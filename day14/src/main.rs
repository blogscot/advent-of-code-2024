mod point;

use point::Point;
use regex::Regex;

type Position = Point;
type Velocity = Point;

#[derive(Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

const QUADRANTS: [Quadrant; 4] = [
    Quadrant::TopLeft,
    Quadrant::TopRight,
    Quadrant::BottomLeft,
    Quadrant::BottomRight,
];

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

struct Bathroom {
    width: u32,
    height: u32,
}

impl Robot {
    fn _new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
    fn advance(&mut self, bathroom: &Bathroom) {
        let Bathroom { width, height } = bathroom;
        self.position = self.position + Position::new(self.velocity.x, self.velocity.y);
        self.position.x = self.position.x.rem_euclid(*width as i32);
        self.position.y = self.position.y.rem_euclid(*height as i32);
    }
}

impl Bathroom {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn in_top_quadrant(&self, point: &Point) -> bool {
        (point.y as u32) < self.height / 2
    }
    fn in_bottom_quadrant(&self, point: &Point) -> bool {
        (point.y as u32) > self.height / 2
    }
    fn in_left_quadrant(&self, point: &Point) -> bool {
        (point.x as u32) < self.width / 2
    }
    fn in_right_quadrant(&self, point: &Point) -> bool {
        (point.x as u32) > self.width / 2
    }

    fn in_quadrant(&self, quadrant: &Quadrant, points: &[Point]) -> u32 {
        let test1 = match quadrant {
            Quadrant::TopLeft | Quadrant::TopRight => Self::in_top_quadrant,
            Quadrant::BottomLeft | Quadrant::BottomRight => Self::in_bottom_quadrant,
        };
        let test2 = match quadrant {
            Quadrant::TopLeft | Quadrant::BottomLeft => Self::in_left_quadrant,
            Quadrant::TopRight | Quadrant::BottomRight => Self::in_right_quadrant,
        };
        points
            .iter()
            .filter(|p| test1(self, p) && test2(self, p))
            .count() as u32
    }
}

fn initialise_robots(data: &[&str]) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots = Vec::new();
    for line in data {
        let caps = re.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().parse().unwrap();
        let vx = caps.get(3).unwrap().as_str().parse().unwrap();
        let vy = caps.get(4).unwrap().as_str().parse().unwrap();
        robots.push(Robot {
            position: Point::new(x, y),
            velocity: Point::new(vx, vy),
        });
    }
    robots
}

fn print_bathroom(robots: &[Robot], bathroom: &Bathroom) {
    for y in 0..bathroom.height {
        for x in 0..bathroom.width {
            let point = Point::new(x as i32, y as i32);
            let num_robots = robots.iter().filter(|r| r.position == point).count();
            if num_robots > 0 {
                print!("{}", num_robots);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn populate(robots: &Vec<Robot>, width: u32, height: u32) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots {
        grid[robot.position.y as usize][robot.position.x as usize] = '#';
    }
    grid
}

fn _display_bathroom(grid: &[Vec<char>], bathroom: &Bathroom) {
    for row in 0..bathroom.height {
        for col in 0..bathroom.width {
            print!("{}", grid[row as usize][col as usize]);
        }
        println!();
    }
}

fn solve_part1(data: &Vec<&str>) -> u32 {
    let mut robots = initialise_robots(data);
    let bathroom = Bathroom::new(101, 103);

    for _ in 0..100 {
        robots.iter_mut().for_each(|robot| robot.advance(&bathroom));
    }

    let final_positions: Vec<Point> = robots.iter().map(|r| r.position).collect();
    QUADRANTS
        .iter()
        .map(|q| bathroom.in_quadrant(q, &final_positions))
        .product()
}

fn detect_easter_egg(grid: &[Vec<char>], bathroom: &Bathroom) -> bool {
    let line_width = 30;
    for y in 0..bathroom.height {
        for x in 0..bathroom.width - line_width {
            if (0..line_width).all(|i| grid[y as usize][x as usize + i as usize] == '#') {
                return true;
            }
        }
    }
    false
}

fn solve_part2(data: &Vec<&str>) -> Option<u32> {
    let mut robots = initialise_robots(data);
    let bathroom = Bathroom::new(101, 103);
    let mut grid : Vec<Vec<char>>;

    for n in 1..10000 {
        robots.iter_mut().for_each(|robot| robot.advance(&bathroom));
        grid = populate(&robots, bathroom.width, bathroom.height);
        if detect_easter_egg(&grid, &bathroom) {
            print_bathroom(&robots, &bathroom);
            return Some(n);
        }
    }
    None
}

fn main() {
    let data: Vec<&str> = include_str!("puzzle.txt").lines().collect();

    println!("Part 1:{:?}", solve_part1(&data));
    println!("Part 2:{:?}", solve_part2(&data).unwrap());
}
