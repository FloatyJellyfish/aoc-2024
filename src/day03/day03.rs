use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

use regex::Regex;

fn main() {
    let file = File::open("src/day03/input.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let re = Regex::new(r"mul\((?<X>[0-9]{1,3}),(?<Y>[0-9]{1,3})\)").unwrap();
    let mut memory = String::new();
    reader.read_to_string(&mut memory).unwrap();

    let mut total: i32 = 0;
    re.captures_iter(&memory).for_each(|caps| {
        let x = i32::from_str(caps.name("X").unwrap().as_str()).unwrap();
        let y = i32::from_str(caps.name("Y").unwrap().as_str()).unwrap();
        total += x * y
    });
    println!("Part 1: {total}");

    let file = File::open("src/day03/input.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let re =
        Regex::new(r"mul\((?<X>[0-9]{1,3}),(?<Y>[0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();
    let mut memory = String::new();
    reader.read_to_string(&mut memory).unwrap();

    let mut enabled = true;
    let mut total: i32 = 0;
    re.captures_iter(&memory).for_each(|caps| {
        if let Some(_) = caps.name("do") {
            enabled = true;
        } else if let Some(_) = caps.name("dont") {
            enabled = false;
        } else if enabled {
            let x = i32::from_str(caps.name("X").unwrap().as_str()).unwrap();
            let y = i32::from_str(caps.name("Y").unwrap().as_str()).unwrap();
            total += x * y
        }
    });

    println!("Part 2: {total}");
}
