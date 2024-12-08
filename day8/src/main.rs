mod point;

use std::collections::{HashMap, HashSet};

use point::Point;

#[derive(Debug)]
struct City {
    grid: Vec<Vec<char>>,
    width: u32,
    height: u32,
}

impl City {
    fn new(grid: Vec<Vec<char>>) -> Self {
        City {
            width: grid[0].len() as u32,
            height: grid.len() as u32,
            grid,
        }
    }

    fn is_valid(&self, point: &Point) -> bool {
        point.row >= 0
            && point.col >= 0
            && point.row < self.height as i32
            && point.col < self.width as i32
    }

    fn find_antennas(&self) -> HashMap<char, Vec<Point>> {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let c =  self.grid[row as usize][col as usize];
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_default()
                        .push(Point::new(row as i32, col as i32));
                }
            }
        }
        antennas
    }

    fn calculate_antenode_positions(
        &self,
        antennas: &HashMap<char, Vec<Point>>,
        frequency: char,
    ) -> Vec<Point> {
        let points = antennas.get(&frequency).unwrap();
        let mut antenodes: Vec<Point> = Vec::new();
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let p1 = points[i];
                let p2 = points[j];
                let diff = p1 - p2;
                antenodes.push(p1 + diff);
                antenodes.push(p2 - diff);
            }
        }
        antenodes.into_iter().filter(|p| self.is_valid(p)).collect()
    }

    fn calculate_antenode_positions_with_harmonics(
        &self,
        antennas: &HashMap<char, Vec<Point>>,
        frequency: char,
    ) -> Vec<Point> {
        let points = antennas.get(&frequency).unwrap();
        let mut antenodes: Vec<Point> = Vec::new();
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let p1 = points[i];
                let p2 = points[j];
                let diff = p1 - p2;
                let mut p = p1 + diff;
                while self.is_valid(&p) {
                    antenodes.push(p);
                    p = p + diff;
                }
                p = p2 - diff;
                while self.is_valid(&p) {
                    antenodes.push(p);
                    p = p - diff;
                }
            }
        }
        antennas.iter().filter(|(_, points)| points.len() > 1).for_each(|(_, points)| {
            antenodes.append(&mut points.clone());
        });
        antenodes
    }
}

fn solve_part1(city: &City) -> usize {
    let antennas = city.find_antennas();
    antennas
        .keys()
        .flat_map(|key| city.calculate_antenode_positions(&antennas, *key))
        .collect::<HashSet<Point>>().len()
}

fn solve_part2(city: &City) -> usize {
    let antennas = city.find_antennas();
    antennas
        .keys()
        .flat_map(|key| city.calculate_antenode_positions_with_harmonics(&antennas, *key))
        .collect::<HashSet<Point>>().len()
}

fn main() {
    let grid: Vec<Vec<char>> = include_str!("puzzle.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let city = City::new(grid);

    println!("Part 1: {:?}", solve_part1(&city));
    println!("Part 2: {:?}", solve_part2(&city));
}
