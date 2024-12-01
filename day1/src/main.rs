use std::collections::HashMap;

fn solve_part1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| i32::abs(*a - *b))
        .sum()
}

fn solve_part2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut frequency_list: HashMap<i32, i32> = HashMap::new();
    for &x in list2.iter() {
        *frequency_list.entry(x).or_insert(0) += 1;
    }

    list1
        .iter()
        .map(|&x| x * *frequency_list.entry(x).or_default())
        .sum()
}

fn main() {
    let input = include_str!("../puzzle.txt");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        list1.push(a.parse::<i32>().unwrap());
        list2.push(b.parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    println!("Part 1: {}", solve_part1(&list1, &list2));
    println!("Part 2: {}", solve_part2(&list1, &list2));

}