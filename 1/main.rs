use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = get_input(&args[1]);

    let sum = sum(&contents);

    let duplicate = find_first_duplicate(&contents);

    println!("{}, {}", sum, duplicate);
}

fn get_input(filename: &str) -> String {
    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read file");

    contents
}

fn sum(input: &String) -> i32 {
     input.split_terminator('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .fold(0, |acc, x| acc + x)
}

fn find_first_duplicate(input: &String) -> i32 {
    let mut seen: Vec<i32> = Vec::new();

    let numbers: Vec<i32> = input.split_terminator('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let mut sum = 0;

    loop {
        for x in &numbers {
            sum += *x;

            for y in &seen {
                if sum == *y {
                    return *y;
                }
            }

            seen.push(sum);
        }
    }
}
