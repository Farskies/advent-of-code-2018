use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::str;


fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = get_input(&args[1]);

    let polymer = create_polymer(&contents);

    println!("{}", polymer.len());
}

fn get_input(filename: &str) -> String {
    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read file");

    contents
}

fn indexes_of_opposite_polarites(polymer: &Vec<u8>) -> Vec<usize> {
    let mut indexes = vec![];
    let mut iter = polymer.iter().enumerate().peekable();

    while let Some((current_i, current)) = iter.next() {
        let mut is_next_opposite = false;

        if let Some((next_i, next)) = iter.peek() {
            is_next_opposite = is_opposite(current, next);
        }

        if is_next_opposite {
            indexes.push(current_i);
            indexes.push(current_i + 1);
            iter.next();
        }
    }

    indexes
}

fn is_opposite(a: &u8, b: &u8) -> bool {
    return (*a as i8 - *b as i8).abs() == 32;
}

fn create_polymer(input: &String) -> String {
    let mut polymer = input.to_owned().into_bytes();

    loop {
        let indexes = indexes_of_opposite_polarites(&polymer);

        if indexes.len() == 0 {
            return String::from_utf8(polymer).unwrap();
        }

        for i in indexes.iter().rev() {
            polymer.remove(*i);
        }
    }
}
