fn check_report(report: &[i32]) -> bool {
    let diffs = report
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<i32>>();

    diffs.iter().all(|diff| diff.abs() >= 1 && diff.abs() <= 3)
        && (diffs.iter().all(|diff| diff > &0) || diffs.iter().all(|diff| diff < &0))
}

fn check_report_with_dampening(report: Vec<i32>) -> bool {
    if check_report(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.clone();
        report.remove(i);
        if check_report(&report) {
            return true;
        }
    }
    false
}

fn solve_part1(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|report| check_report(report)).count()
}

fn solve_part2(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| check_report_with_dampening(report.to_vec()))
        .count()
}

fn main() {
    let input = include_str!("../puzzle.txt");

    let mut reports = Vec::new();
    for line in input.lines() {
        let mut report = Vec::new();
        for num in line.split_whitespace() {
            report.push(num.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    println!("{:?}", solve_part1(&reports));

    println!("{:?}", solve_part2(&reports));
}
