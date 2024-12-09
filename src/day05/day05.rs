use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

fn main() {
    let file = File::open("src/day05/input.txt").expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    let (rules_raw, updates_raw) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules_raw.split("\n") {
        let rule = rule.trim();
        let (dependency, number) = rule.split_once("|").unwrap();
        let dependency = u32::from_str(dependency).unwrap();
        let number = u32::from_str(number).unwrap();
        if rules.contains_key(&number) {
            rules.get_mut(&number).unwrap().push(dependency);
        } else {
            rules.insert(number, vec![dependency]);
        }
    }

    let updates: Vec<&str> = updates_raw.split("\n").collect();
    let updates: Vec<Vec<&str>> = updates
        .iter()
        .map(|update| update.split(",").collect())
        .collect();
    let updates: Vec<Vec<u32>> = updates
        .iter()
        .map(|update| {
            update
                .iter()
                .map(|page| u32::from_str(page).unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;
    for update in updates {
        let mut in_order = true;
        for page in update.iter() {
            if let Some(dependencies) = rules.get(&page) {
                for dependency in dependencies {
                    if update.contains(&dependency) {
                        if update.iter().position(|p| p == dependency).unwrap()
                            > update.iter().position(|p| p == page).unwrap()
                        {
                            in_order = false;
                        }
                    }
                }
            }
        }
        if in_order {
            let middle = update[(update.len()) / 2];
            total += middle;
        }
    }

    println!("Part 1: {total}");
}
