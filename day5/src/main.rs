#[derive(Debug)]
struct Rule(u32, u32);

impl Rule {
    fn new(s: &str) -> Self {
        let (a, b) = s.split_once('|').unwrap();
        Rule(a.parse().unwrap(), b.parse().unwrap())
    }

    // If a rule page order does not match the update the rule is invalid.
    fn check(&self, update: &[u32]) -> bool {
        let index1 = update.iter().position(|&x| x == self.0);
        let index2 = update.iter().position(|&x| x == self.1);
        match (index1, index2) {
            (Some(a), Some(b)) => a < b,
            _ => true,
        }
    }

    fn fix(&self, update: &mut Vec<u32>) {
        let index1 = update.iter().position(|&x| x == self.0);
        let index2 = update.iter().position(|&x| x == self.1);
        match (index1, index2) {
            (Some(a), Some(b)) => {
                if b < a {
                    update.swap(a, b);
                }
            }
            _ => {}
        }
    }
}

fn check_rules(rules: &[Rule], update: &[u32]) -> bool {
    rules.iter().all(|rule| rule.check(update))
}

fn fix_rules(rules: &[Rule], update: &mut Vec<u32>) {
    rules.iter().for_each(|rule| rule.fix(update));
}

fn solve_part1(rules: &[Rule], updates: &[Vec<u32>]) -> u32 {
    updates
        .iter()
        .filter(|update| check_rules(&rules, update))
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

fn solve_part2(rules: &[Rule], updates: &mut [Vec<u32>]) -> u32 {
    let mut failing: Vec<&mut Vec<u32>> = updates
        .iter_mut()
        .filter(|update| !check_rules(&rules, update))
        .collect();

    failing
        .iter_mut()
        .for_each(|update| {
            while !check_rules(&rules, update) {
                fix_rules(&rules, update)
            }
        });

    failing
        .iter()
        .map(|update| update[(update.len() - 1) / 2])
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

    let mut updates: Vec<_> = updates;
    println!("Part 2: {:?}", solve_part2(&rules, &mut updates));
}
