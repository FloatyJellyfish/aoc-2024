use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let file = File::open("src/day01/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim().to_string();
        let mut split = line.split("   ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        let a = i32::from_str(a).unwrap();
        let b = i32::from_str(b).unwrap();
        first.push(a);
        second.push(b);
    }

    first.sort();
    second.sort();

    assert_eq!(first.len(), second.len());

    let mut total_distance = 0;
    for i in 0..first.len() {
        let distance = i32::abs(first[i] - second[i]);
        total_distance += distance;
    }

    println!("Part 1: {total_distance}");

    let mut occurrences = HashMap::new();
    let mut simularity_score = 0;
    for i in 0..first.len() {
        if !occurrences.contains_key(&first[i]) {
            let mut count = 0;
            for j in 0..second.len() {
                if first[i] == second[j] {
                    count += 1;
                }
            }
            occurrences.insert(first[i], first[i] * count);
        }
        simularity_score += occurrences.get(&first[i]).unwrap();
    }

    println!("Part 2: {simularity_score}");
}
