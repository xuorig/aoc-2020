use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();

    let reader = BufReader::new(file);

    let noop_regex = Regex::new(r"contain no other bags").unwrap();
    let container_regex = Regex::new(r"(\w+ \w+) bags contain").unwrap();
    let contains_regex = Regex::new(r"(\d) (\w+\s\w+) bag").unwrap();

    let mut bag_definitions: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for (_, line) in reader.lines().enumerate() {
        let rule = line.unwrap();

        if !noop_regex.is_match(&rule) {
            let h = container_regex.captures(&rule).unwrap();
            let container = h.get(1).unwrap().as_str();

            for cap in contains_regex.captures_iter(&rule) {
                let quantity = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let color = cap.get(2).unwrap().as_str();

                match bag_definitions.entry(container.to_string()) {
                    Entry::Occupied(mut o) => {
                        o.get_mut().insert(color.to_string(), quantity);
                    }
                    Entry::Vacant(v) => {
                        let mut quantities: HashMap<String, i32> = HashMap::new();
                        quantities.insert(color.to_string(), quantity);
                        v.insert(quantities);
                    }
                };
            }
        }
    }

    let mut contained = 0;

    for (_, kv) in bag_definitions.iter().enumerate() {
        if contains_shiny_gold_bag(&bag_definitions, kv.0) {
            contained += 1;
        }
    }

    println!("PART I: {}", contained);
}

fn contains_shiny_gold_bag(
    bag_definitions: &HashMap<String, HashMap<String, i32>>,
    color: &str,
) -> bool {
    let fits_in_bag = bag_definitions.get(&color.to_string());

    if fits_in_bag.is_some() {
        for (_, bag) in fits_in_bag.unwrap().iter().enumerate() {
            if bag.0 == "shiny gold" {
                return true;
            }
            if contains_shiny_gold_bag(bag_definitions, bag.0) {
                return true;
            }
        }
        false
    } else {
        false
    }
}
