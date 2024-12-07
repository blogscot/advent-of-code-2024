fn calculate(nums: &mut [u64], acc: &mut Vec<u64>, operators: &[&str]) {
    if nums.len() == 1 {
        acc.push(nums[0]);
        return;
    }
    let first = nums[0];
    let second = nums[1];
    let nums = nums[2..].to_vec();

    for op in operators {
        let mut nums = nums.clone();
        let result = match *op {
            "+" => first + second,
            "*" => first * second,
            "||" => format!("{}{}", first, second).parse().unwrap(),
            _ => 0,
        };
        nums.insert(0, result);
        calculate(&mut nums, acc, operators);
    }
}

fn solve(equations: &mut [(u64, Vec<u64>)], operators: &[&str]) -> u64 {
    equations.iter_mut().map(|(total, nums)| {
        let mut acc = vec![];
        calculate(nums, &mut acc, &operators);
        if acc.contains(total) {
            *total
        }  else {
            0
        }
    })
    .sum::<u64>()
}

fn solve_part1(equations: &mut [(u64, Vec<u64>)]) -> u64 {
    let operators = vec!["+", "*"];
    solve(equations, &operators)
}

fn solve_part2(equations: &mut [(u64, Vec<u64>)]) -> u64 {
    let operators = vec!["+", "*", "||"];
    solve(equations, &operators)
}

fn main() {
    let lines: Vec<&str> = include_str!("puzzle.txt").lines().collect();
    let mut equations: Vec<(u64, Vec<u64>)> = lines
        .iter()
        .map(|s| s.split_once(": ").unwrap())
        .map(|(total, nums)| {
            (
                total.parse().unwrap(),
                nums.split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect();

    println!("Part 1: {}", solve_part1(&mut equations));
    println!("Part 2: {}", solve_part2(&mut equations));
}
