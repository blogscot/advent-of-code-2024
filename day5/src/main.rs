#[derive(Debug)]
struct Rule(u32, u32);

impl Rule {
    fn new(s: &str) -> Self {
        let (a, b) = s.split_once('|').unwrap();
        Rule(a.parse().unwrap(), b.parse().unwrap())
    }

    // If the update does not match the rule page order the update is invalid.
    fn check(&self, update: &[u32]) -> bool {
        let index1 = update.iter().position(|&x| x == self.0);
        let index2 = update.iter().position(|&x| x == self.1);
        if let (Some(a), Some(b)) = (index1, index2) {
            return a < b
        }
        true
    }

    fn fix(&self, update: &mut [u32]) {
        let index1 = update.iter().position(|&x| x == self.0);
        let index2 = update.iter().position(|&x| x == self.1);
        if let (Some(a), Some(b)) = (index1, index2) {
            if b < a {
                update.swap(a, b);
            }
        }
    }
}

fn check_rules(rules: &[Rule], update: &[u32]) -> bool {
    rules.iter().all(|rule| rule.check(update))
}

fn fix_rules(rules: &[Rule], update: &mut [u32]) {
    rules.iter().for_each(|rule| rule.fix(update));
}

fn solve_part1(rules: &[Rule], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| check_rules(rules, update))
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

fn solve_part2(rules: &[Rule], updates: &[Vec<u32>]) -> u32 {
    let mut failing: Vec<Vec<u32>> = updates
        .iter()
        .filter(|update| !check_rules(rules, update))
        .cloned()
        .collect();

    failing
        .iter_mut()
        .map(|update | {
            while !check_rules(rules, update) {
                fix_rules(rules, update)
            }
            update[(update.len() - 1) / 2]
        })
        .sum()
}

fn main() {
    let (rules, updates) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let rules: Vec<_> = rules.split_whitespace().map(Rule::new).collect();
    let updates: Vec<_> = updates
        .split_whitespace()
        .map(|update| {
            update
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    println!("Part 1: {:?}", solve_part1(&rules, &updates));
    println!("Part 2: {:?}", solve_part2(&rules, &updates));
}
