use regex::Regex;

fn solve_part1(memory: &str) -> u64 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [first, second]) in re.captures_iter(memory).map(|c| c.extract()) {
        let first = first.parse::<u64>().unwrap();
        let second = second.parse::<u64>().unwrap();
        sum += first * second;
    }
    sum
}

fn solve_part2(memory: &str) -> u64 {
    let mut sum = 0;
    let mut state = State::On;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();
    for (operation, first, second) in re
        .captures_iter(memory)
        .map(|m| (m.get(0).unwrap().as_str(), m.get(1), m.get(2)))
    {
        match operation {
            "do()" => state = State::On,
            "don't()" => state = State::Off,
            _ => {
                if state == State::On {
                    let first = first.unwrap().as_str().parse::<u64>().unwrap();
                    let second = second.unwrap().as_str().parse::<u64>().unwrap();
                    sum += first * second;
                }
            }
        }
    }
    sum
}

#[derive(Debug, PartialEq)]
enum State {
    On,
    Off,
}

fn main() {
    let memory = include_str!("../src/puzzle.txt");

    println!("Part 1: {}", solve_part1(memory));
    println!("Part 2: {}", solve_part2(memory));
}
