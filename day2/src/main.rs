use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut valid_passwords_part_1 = 0;
    let mut valid_passwords_part_2 = 0;

    // Part 1
    for (_, line) in reader.lines().enumerate() {
        let val = line.unwrap();
        let mut parts = val.split_whitespace();

        let range_part = parts.next();
        let mut range_values = range_part.unwrap().split("-");
        let min = range_values.next().unwrap().parse::<i32>().unwrap();
        let max = range_values.next().unwrap().parse::<i32>().unwrap();

        let letter_part = parts.next();
        let letter = letter_part.unwrap().chars().next().unwrap();

        let password = parts.next().unwrap();

        if is_valid(min, max, letter, password) {
            valid_passwords_part_1 += 1
        }

        if is_valid_part_2(min, max, letter, password) {
            valid_passwords_part_2 += 1
        }
    }

    println!("Valid passwords part 1: {}", valid_passwords_part_1);
    println!("Valid passwords part 2: {}", valid_passwords_part_2);
}

fn is_valid(min: i32, max: i32, letter: char, password: &str) -> bool {
    let mut letter_count = 0;

    for c in password.chars() {
        if c == letter {
            letter_count += 1;
        }
    }

    letter_count >= min && letter_count <= max
}

// Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character,
// and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter.
// Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
fn is_valid_part_2(first: i32, second: i32, letter: char, password: &str) -> bool {
    let first_letter = password.chars().nth(first as usize - 1).unwrap();
    let second_letter = password.chars().nth(second as usize - 1).unwrap();

    (first_letter == letter) ^ (second_letter == letter)
}
