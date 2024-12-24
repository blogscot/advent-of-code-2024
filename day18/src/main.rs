mod point;
mod tile;

use phf::phf_map;
use point::Point;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use tile::Tile;

struct Memory {
    start: Point,
    end: Point,
    layout: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

type Direction = phf::Map<char, (i32, i32)>;

const DIRECTIONS: Direction = {
    phf_map! {
        'N' => (0, -1),
        'S' => (0, 1),
        'E' => (1, 0),
        'W' => (-1, 0),
    }
};

impl Memory {
    fn new(width: usize, height: usize) -> Memory {
        Memory {
            layout: vec![vec![Tile::Floor(None); width + 1]; height + 1],
            start: Point::default(),
            end: Point::new(width as i32, height as i32),
            width,
            height,
        }
    }
    fn get(&self, point: Point) -> Option<Tile> {
        if self.is_valid(point) {
            Some(self.layout[point.y as usize][point.x as usize])
        } else {
            None
        }
    }
    fn is_valid(&self, point: Point) -> bool {
        let Point { x, y } = point;
        x >= 0 && y >= 0 && x <= self.width as i32 && y <= self.height as i32
    }
    fn set(&mut self, point: Point, tile: Tile) {
        self.layout[point.y as usize][point.x as usize] = tile;
    }
    fn neighbours(&self, point: Point) -> Vec<Point> {
        DIRECTIONS
            .values()
            .map(|&delta| point + delta.into())
            .filter(|point| self.is_valid(*point))
            .collect()
    }
    fn search(&mut self) {
        let mut queue = PriorityQueue::new();
        queue.push((0, self.start), Reverse(0));
        self.set(self.start, Tile::Floor(Some(0)));
        while let Some(((steps, point), _)) = queue.pop() {
            if point == self.end {
                println!("Found: {} steps", steps);
                return;
            }
            self.neighbours(point).iter().for_each(|neighbour| {
                if let Some(Tile::Floor(value)) = self.get(*neighbour) {
                    if value.is_none() || value.unwrap() > steps {
                        self.set(*neighbour, Tile::Floor(Some(steps + 1)));
                        queue.push((steps + 1, *neighbour), Reverse(steps + 1));
                    }
                }
            })
        }
    }
    fn drop_bytes(&mut self, bytes: Vec<Point>) {
        bytes.iter().for_each(|point| self.set(*point, Tile::Wall));
    }
    fn dump(&self) {
        self.layout.iter().for_each(|row| {
            for tile in row {
                print!("{}", tile);
            }
            println!();
        });
        println!();
    }
}

fn main() {
    let data: Vec<Point> = include_str!("puzzle.txt")
        .lines()
        .map(Point::from)
        .collect();

    let (width, height) = data.iter().fold((0, 0), |(max_x, max_y), Point { x, y }| {
        (max_x.max(*x), max_y.max(*y))
    });

    let mut memory = Memory::new(width as usize, height as usize);
    let amount = 1024;
    memory.drop_bytes(data.into_iter().take(amount).collect());
    memory.search();
    memory.dump();
}
