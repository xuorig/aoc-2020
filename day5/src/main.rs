use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut passes: Vec<(String, String)> = vec![];

    for (_, line) in reader.lines().enumerate() {
        let position = line.unwrap();
        // This works because we know input is ASCII
        let (row, col) = position.split_at(7);
        passes.push((String::from(row), String::from(col)))
    }

    // Not sure if this is much better than running binary search on all of them
    // but let's try sorting the rows and cols to find the top, then compute
    passes.sort_by(|a, b| {
        let row_cmp = a.0.cmp(&b.0);
        // Reverse sort for this one
        let col_cmp = b.1.cmp(&a.1);

        match row_cmp {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => col_cmp,
        }
    });

    let top = &passes[0];
    println!("{:?}", top);
    println!("{}", seat_id(&top.0, &top.1));
}

fn seat_id(row: &String, col: &String) -> i32 {
    let mut row_min = 0;
    let mut row_max = 127;

    for (_, r) in row.chars().enumerate() {
        if r == 'B' {
            row_min = ((row_max + row_min) / 2) + 1;
        } else {
            row_max = ((row_max + row_min) / 2) - 1;
        }

        println!("min {} max {}", row_min, row_max);
    }

    let mut col_min = 0;
    let mut col_max = 7;

    for (_, r) in col.chars().enumerate() {
        if r == 'R' {
            col_min = ((col_max + col_min) / 2) + 1;
        } else {
            col_max = ((col_max + col_min) / 2) - 1;
        }
    }

    println!("row {} col {}", row_min, col_min);

    // Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.
    (row_min * 8) + col_min
}
