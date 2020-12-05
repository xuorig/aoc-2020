use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    let mut required_fields: HashSet<String> = HashSet::new();
    required_fields.insert(String::from("byr"));
    required_fields.insert(String::from("iyr"));
    required_fields.insert(String::from("eyr"));
    required_fields.insert(String::from("hgt"));
    required_fields.insert(String::from("hcl"));
    required_fields.insert(String::from("ecl"));
    required_fields.insert(String::from("pid"));
    // "cid" is optional

    let file = File::open("src/input.txt").unwrap();

    let reader = BufReader::new(file);
    let mut valid_passports = 0;
    let mut valid_strict_passports = 0;

    let mut current_passport: HashMap<String, String> = HashMap::new();

    for (_, line) in reader.lines().enumerate() {
        let current_line = line.unwrap().to_string();

        if current_line.is_empty() {
            if valid_passport(&current_passport, &required_fields) {
                valid_passports += 1;
            }

            if valid_passport_strict(&current_passport, &required_fields) {
                valid_strict_passports += 1;
            }

            // Start of a new passport
            current_passport = HashMap::new();
        } else {
            let fields = current_line.split_whitespace();
            for (_, field) in fields.enumerate() {
                let mut field_data = field.split(":");
                current_passport.insert(
                    field_data.next().unwrap().to_string(),
                    field_data.next().unwrap().to_string(),
                );
            }
        }
    }

    if valid_passport(&current_passport, &required_fields) {
        valid_passports += 1;
    }

    if valid_passport_strict(&current_passport, &required_fields) {
        valid_strict_passports += 1;
    }

    println!("Part 1: Valid passports: {}", valid_passports);
    println!("Part 2: Valid passports: {}", valid_strict_passports)
}

fn valid_passport(passport: &HashMap<String, String>, required_fields: &HashSet<String>) -> bool {
    required_fields.iter().all(|k| passport.contains_key(k))
}

fn valid_passport_strict(
    passport: &HashMap<String, String>,
    required_fields: &HashSet<String>,
) -> bool {
    // First check that all required fields are present
    if !required_fields.iter().all(|k| passport.contains_key(k)) {
        println!("Failed required all keys");
        return false;
    }

    if !valid_year(passport.get("byr").unwrap(), 1920, 2002) {
        println!("Failed byr");
        return false;
    }

    if !valid_year(passport.get("iyr").unwrap(), 2010, 2020) {
        println!("Failed iyr");
        return false;
    }

    if !valid_year(passport.get("eyr").unwrap(), 2020, 2030) {
        println!("Failed eyr");
        return false;
    }

    if !valid_height(passport.get("hgt").unwrap()) {
        println!("Failed hgt");
        return false;
    }

    if !valid_hair(passport.get("hcl").unwrap()) {
        println!("Failed hcl");
        return false;
    }

    if !valid_eyes(passport.get("ecl").unwrap()) {
        println!("Failed ecl");
        return false;
    }

    if !valid_pid(passport.get("pid").unwrap()) {
        println!("Failed pid");
        return false;
    }

    println!("Valid Passport");
    true
}

fn valid_year(year: &String, min: i32, max: i32) -> bool {
    let parsed_year = year.parse::<i32>().unwrap();
    parsed_year >= min && parsed_year <= max
}

fn valid_height(height: &String) -> bool {
    let cm_re = Regex::new(r"(\d+)cm").unwrap();
    if cm_re.is_match(height) {
        let h = cm_re.captures(height).unwrap();
        let parsed_height = h.get(1).unwrap().as_str().parse::<i32>();

        match parsed_height {
            Ok(x) => return x >= 150 && x <= 193,
            Err(_) => return false,
        }
    }

    let in_re = Regex::new(r"(\d+)in").unwrap();

    if in_re.is_match(height) {
        let h = in_re.captures(height).unwrap();
        let parsed_height = h.get(1).unwrap().as_str().parse::<i32>();

        match parsed_height {
            Ok(x) => return x >= 59 && x <= 76,
            Err(_) => return false,
        }
    }

    false
}

fn valid_hair(hair: &String) -> bool {
    let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    re.is_match(hair)
}

fn valid_eyes(eyes: &String) -> bool {
    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth){1}$").unwrap();
    re.is_match(eyes)
}

fn valid_pid(pid: &String) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(pid)
}
