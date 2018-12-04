extern crate edit_distance;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::cmp::max;
use edit_distance::edit_distance;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = get_input(&args[1]);

    let checksum = checksum(&contents);

    let box_name = box_name(&contents);

    println!("{}, {:?}", checksum, box_name);
}

fn get_input(filename: &str) -> String {
    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read file");

    contents
}

fn checksum(input: &str) -> u32 {
    let counts = input.split_terminator('\n')
        .fold((0u32, 0u32),  |acc, line| {
            let chars: Vec<_> = line.chars().collect();

            let mut unique_chars = chars.clone();
            unique_chars.dedup_by(|a, b| a == b);

            let mut duplicate = 0;
            let mut triplicate = 0;

            for unique_char in &unique_chars {
                let mut char_count = 0;

                for char in &chars {
                    if char == unique_char {
                        char_count += 1;
                    }
                }

                match char_count {
                    2 => duplicate = max(duplicate, 1),
                    3 => triplicate = max(triplicate, 1),
                    _ => {}
                }
            }

            (acc.0 + duplicate, acc.1 + triplicate)
        });

    counts.0 * counts.1
}

fn box_name(input: &str) -> Option<String> {
    let lines: Vec<_> = input.split_terminator('\n').collect();

    for line_x in &lines {
        for line_y in &lines {
            if edit_distance(line_x, line_y) == 1 {
                return Some(remove_diff(line_x, line_y));
            }
        }
    }

    return None;
}

fn remove_diff(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .fold("".to_string(), |mut acc, (char_a, char_b)| {
            if char_a == char_b {
                acc.push(char_a);
            }

            acc
        })
}
