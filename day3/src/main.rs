use regex::Regex;

fn solve_part1(memory: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.find_iter(memory).map(|m| m.as_str()).collect::<Vec<&str>>();

    let mut sum = 0;
    for m in matches {
        re2.captures(m).map(|c| c.extract()).into_iter().for_each(|(_, [first, second])| {
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();
            sum += first * second;
        });
    }
    sum
}

fn solve_part2(memory: &str) -> u64 {
    let mut sum = 0;
    let mut state = State::On;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(memory).map(|m| m.get(0).unwrap().as_str());
    for m in matches {
        match m {
            "do()" => state = State::On,
            "don't()" => state = State::Off,
            _ => {
                if state == State::On {
                re2.captures(m)
                    .map(|c| c.extract())
                    .into_iter()
                    .for_each(|(_, [first, second])| {
                        let first = first.parse::<u64>().unwrap();
                        let second = second.parse::<u64>().unwrap();
                        sum += first * second;
                    })
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
