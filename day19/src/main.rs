fn eat(design: &str, pattern: &str) -> Option<String> {
    design
        .strip_prefix(pattern)
        .map(|stripped| stripped.to_string())
}

fn check(design: &str, patterns: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }
    for pattern in patterns {
        if let Some(fabric) = eat(design, pattern) {
            if check(&fabric, patterns) {
                return true;
            }
        };
    }
    false
}

fn main() {
    let (patterns, designs) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let designs = designs.split("\n").collect::<Vec<_>>();

    let result: usize = designs
        .into_iter()
        .filter(|design| check(design, &patterns))
        .count();

    println!("Part 1: {:?}", result);
}
