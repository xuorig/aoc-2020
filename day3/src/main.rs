use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut map: Vec<Vec<bool>> = vec![];

    for (_, line) in reader.lines().enumerate() {
        let mut row: Vec<bool> = vec![];

        for c in line.unwrap().chars() {
            row.push(c == '#');
        }

        map.push(row)
    }

    // part 1
    println!("Hit trees: {}", travel(&map, 1, 3));
    // part 2
    // There's probably something smarter here... :shrug:
    println!(
        "Hit trees: {}",
        travel(&map, 1, 1)
            * travel(&map, 1, 3)
            * travel(&map, 1, 5)
            * travel(&map, 1, 7)
            * travel(&map, 2, 1)
    );
}

fn travel(map: &Vec<Vec<bool>>, down: usize, right: usize) -> i64 {
    let mut hit_trees = 0;

    let width = map[0].len();

    let mut x = 0;
    let mut y = 0;

    while y < map.len() {
        if map[y][x % width] {
            hit_trees += 1;
        }

        y += down;
        x += right;
    }

    hit_trees
}
