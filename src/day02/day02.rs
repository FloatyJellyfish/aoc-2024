use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let file = File::open("src/day02/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut safe_reports = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let report: Vec<i32> = line
            .trim()
            .split(" ")
            .map(|level| i32::from_str(level).unwrap())
            .collect();
        if is_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("Part 1: {safe_reports}");
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe(report: &[i32]) -> bool {
    let mut direction = None;
    for i in 1..report.len() {
        match direction {
            None => {
                if report[i - 1] < report[i] {
                    direction = Some(Direction::Increasing);
                } else {
                    direction = Some(Direction::Decreasing);
                }
            }
            Some(ref direction) => {
                if *direction == Direction::Increasing && report[i] < report[i - 1] {
                    return false;
                }
                if *direction == Direction::Decreasing && report[i] > report[i - 1] {
                    return false;
                }
            }
        }
        let difference = i32::abs(report[i - 1] - report[i]);
        if difference < 1 || difference > 3 {
            return false;
        }
    }
    true
}
