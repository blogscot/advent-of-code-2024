#[derive(Debug, Eq, PartialEq, Hash)]
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
            if !self.is_valid((x, y)) || self.letters[y as usize][x as usize] != *letter {
                return false;
            }
            let (dx, dy) = (direction.0, direction.1);
            y += dy;
            x += dx;
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
            return (self.at(before) == 'M' && self.at(after) == 'S')
                || (self.at(before) == 'S' && self.at(after) == 'M');
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

fn solve_part1(puzzle: &Puzzle) -> usize {
    (0..puzzle.height)
    .flat_map(|row| (0..puzzle.width)
    .flat_map(move |col| {
        let point = Point { x: col, y: row };
        DIRECTIONS.iter().filter(move |direction| {
            puzzle.find_xmas(&point, &direction)
        })
    })).count()
}

fn solve_part2(puzzle: &Puzzle) -> usize {
    (0..puzzle.height)
    .flat_map(|row| (0..puzzle.width)
    .filter(move |col| {
        let point = Point { x: *col, y: row };
        [UP_LEFT, UP_RIGHT].iter().all(move |direction| {
            puzzle.find_mas(&point, direction)
        })
    })).count()
}

fn main() {
    let input = include_str!("puzzle.txt");
    let letters = input.lines().map(|l| l.chars().collect()).collect();
    let puzzle = Puzzle::new(letters);

    println!("Part 1: {}", solve_part1(&puzzle));
    println!("Part 2: {}", solve_part2(&puzzle));
}
