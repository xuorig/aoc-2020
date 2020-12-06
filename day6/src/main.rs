use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("src/input.txt")
        .unwrap()
        .parse()
        .unwrap();

    let parts: Vec<_> = contents.split("\n\n").collect();

    let res = parts
        .iter()
        .fold(0, |acc, answers| acc + unique_chars(answers));

    let res2 = parts
        .iter()
        .fold(0, |acc, answers| acc + common_answers(answers));

    println!("{}", res);
    println!("{}", res2);
}

fn unique_chars(s: &str) -> usize {
    let mut hs = HashSet::new();

    for (_, c) in s.trim().chars().enumerate() {
        if c.is_alphabetic() {
            hs.insert(c);
        }
    }

    hs.len()
}

fn common_answers(s: &str) -> usize {
    let mut hm: HashMap<char, usize> = HashMap::new();
    let answers: Vec<_> = s.split_whitespace().collect();

    for (_, a) in answers.iter().enumerate() {
        for (_, c) in a.chars().enumerate() {
            match hm.entry(c) {
                Entry::Occupied(mut o) => o.insert(o.get() + 1),
                Entry::Vacant(v) => *v.insert(1),
            };
        }
    }

    hm.values().fold(
        0,
        |acc, val| {
            if *val == answers.len() {
                acc + 1
            } else {
                acc
            }
        },
    )
}
