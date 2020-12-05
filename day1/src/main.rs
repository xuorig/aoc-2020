use std::collections::HashSet;
use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut set: HashSet<i32> = HashSet::new();

    // Preprocess into a hashset
    for (_, line) in reader.lines().enumerate() {
        set.insert(line.unwrap().parse::<i32>().unwrap());
    }

    part_one(&set);
    part_two(&set);
}

fn part_one(number_set: &HashSet<i32>) {
    let addends = find_addends(2020, number_set);

    match addends {
        Some(x) => println!("Num 1: {} Num 2: {} Multiplied: {}", x.0, x.1, x.0 * x.1),
        None => println!("Could not find answer"),
    }
}

fn part_two(number_set: &HashSet<i32>) {
    for (_, num) in number_set.iter().enumerate() {
        let wanted = 2020 - num;

        let rest_addends = find_addends(wanted, number_set);

        match rest_addends {
            Some(x) => println!(
                "Num 1: {} Num 2: {} Num 3: {} Multiplied: {}",
                num,
                x.0,
                x.1,
                num * x.0 * x.1
            ),
            None => {}
        }
    }
}

fn find_addends(sum: i32, number_set: &HashSet<i32>) -> Option<(i32, i32)> {
    for (_, num) in number_set.iter().enumerate() {
        let wanted = sum - num;

        if number_set.contains(&wanted) {
            return Some((*num, wanted));
        }
    }

    None
}
