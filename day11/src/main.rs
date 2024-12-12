use std::collections::HashMap;

// const INPUT: &str = "125 17";
const INPUT: &str = "0 89741 316108 7641 756 9 7832357 91";

type Stone = u64;

fn blink(num: u64) -> Vec<Stone> {
    if num == 0 {
        return vec![1];
    }
    let num_digits = num.ilog10() + 1;
    if num_digits % 2 == 0 {
        let half = num_digits / 2;
        vec![num / 10u64.pow(half), num % 10u64.pow(half)]
    } else {
        vec![num * 2024]
    }
}

fn solve(stones: &Vec<Stone>, num_blinks: u32) -> u64 {
    let mut counter: HashMap<Stone, u64> = HashMap::new();

    stones.iter().for_each(|stone| {
        *counter.entry(*stone).or_insert(0) += 1;
    });

    // Calculate the next generation
    // Store the unique values with their counts in a hashmap
    // Calculate the next generation:
    // Each new stone carries over the count from the previous generation due to duplicate entries.
    // Similarly this new generation contains duplicate stones: their counts are added together.

    for _ in 0..num_blinks {
        let generation: Vec<(Stone, u64)> = counter
            .keys()
            .flat_map(|stone| {
                blink(*stone)
                    .iter()
                    .map(|new_stone| (*new_stone, *counter.get(stone).unwrap_or(&1)))
                    .collect::<Vec<(Stone, u64)>>()
            })
            .collect();

        let mut summer: HashMap<Stone, u64> = HashMap::new();
        for (stone, count) in generation {
            *summer.entry(stone).or_insert(0) += count
        }
        counter = summer;
    }
    counter.values().sum()
}

fn main() {
    let stones: Vec<Stone> = INPUT
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("Part 1: {}", solve(&stones, 25));
    println!("Part 2: {}", solve(&stones, 75));
}
