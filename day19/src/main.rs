use std::collections::HashMap;

fn check(design: &str, patterns: &[&str]) -> usize {
    if design.is_empty() {
        return 1;
    }
    let count = 0;
    for pattern in patterns {
        if let Some(fabric) = design.strip_prefix(pattern) {
            if check(fabric, patterns) == 1 {
                return count + 1;
            }
        }
    }
    count
}

fn check2(cache: &mut HashMap<String, usize>, design: &str, patterns: &[&str]) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(design) {
        return *count;
    }
    let mut count = 0;
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            count += check2(cache, rest, patterns);
        }
    }
    cache.insert(design.to_string(), count);
    count
}

fn main() {
    let (patterns, designs) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let designs = designs.split("\n").collect::<Vec<_>>();

    let part1: usize = designs.iter().map(|design| check(design, &patterns)).sum();

    println!("Part 1: {:?}", part1);

    let mut cache = HashMap::new();
    let part2: usize = designs
        .iter()
        .map(|design| check2(&mut cache, design, &patterns))
        .sum();

    println!("Part 2: {:?}", part2);
}
