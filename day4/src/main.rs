use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

struct Direction(i32, i32);

const UP: Direction = Direction(0, -1);
const DOWN: Direction = Direction(0, 1);
const LEFT: Direction = Direction(-1, 0);
const RIGHT: Direction = Direction(1, 0);
const UP_LEFT: Direction = Direction(-1, -1);
const UP_RIGHT: Direction = Direction(1, -1);
const DOWN_LEFT: Direction = Direction(-1, 1);
const DOWN_RIGHT: Direction = Direction(1, 1);

const DIRECTIONS: [Direction; 8] = [
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

struct Puzzle {
    letters: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn new(letters: Vec<Vec<char>>) -> Self {
        Self {
            width: letters[0].len(),
            height: letters.len(),
            letters,
        }
    }
    fn find_xmas(&self, start: &Point, direction: &Direction) -> bool {
        const XMAS: &[char; 4] = &['X', 'M', 'A', 'S'];
        let Point { x, y } = *start;
        let mut x = x as i32;
        let mut y = y as i32;
        for letter in XMAS.iter() {
            if !self.is_valid((x, y)) {
                return false;
            }
            if self.letters[y as usize][x as usize] != *letter {
                return false;
            }
            let (dx, dy) = (direction.0, direction.1);
            y = y as i32 + dy;
            x = x as i32 + dx;
        }
        true
    }

    fn at(&self, point: (i32, i32)) -> char {
        self.letters[point.1 as usize][point.0 as usize]
    }

    fn find_mas(&self, start: &Point, direction: &Direction) -> bool {
        let Point { x, y } = *start;
        let (delta_x, delta_y) = (direction.0, direction.1);
        if self.letters[y][x] == 'A' {
            let before = (x as i32 - delta_x, y as i32 - delta_y);
            let after = (x as i32 + delta_x, y as i32 + delta_y);
            if !self.is_valid(before) || !self.is_valid(after) {
                return false;
            }
            if (self.at(before) == 'M' && self.at(after) == 'S')
                || (self.at(before) == 'S' && self.at(after) == 'M')
            {
                return true;
            }
        }
        false
    }

    fn is_valid(&self, point: (i32, i32)) -> bool {
        let (x, y) = point;
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        true
    }
}

fn solve_part1(puzzle: &Puzzle) -> u32 {
    let mut count = 0;
    for row in 0..puzzle.height {
        for col in 0..puzzle.width {
            for direction in DIRECTIONS {
                if puzzle.find_xmas(&Point { x: col, y: row }, &direction) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_part2(puzzle: &Puzzle) -> usize {
    let mut points: HashMap<Point, u32> = HashMap::new();
    for row in 0..puzzle.height {
        for col in 0..puzzle.width {
            for direction in [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT] {
                if puzzle.find_mas(&Point { x: col, y: row }, &direction) {
                    let point = Point { x: col, y: row };
                    points.entry(point).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }
    }
    points.iter().filter(|(_, value)| **value == 4).count()
}

fn main() {
    let input = include_str!("puzzle.txt");
    let letters = input.lines().map(|l| l.chars().collect()).collect();
    let puzzle = Puzzle::new(letters);

    println!("Part 1: {}", solve_part1(&puzzle));
    println!("Part 2: {}", solve_part2(&puzzle));
}
