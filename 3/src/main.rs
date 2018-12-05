extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::env;

#[derive(Debug)]
struct Claim {
    id: u32,
    coords: (u32, u32),
    size: (u32, u32)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = get_input(&args[1]);

    let claims = parse_claims(&contents);

    let area = calculate_overlapping_area(&claims);

    let intact_claim = get_intact_claim(&claims);

    println!("{}, {:?}", area, intact_claim);
}

fn get_input(filename: &str) -> String {
    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read file");

    contents
}

fn parse_claims(input: &str) -> Vec<Claim> {
    let regex = Regex::new(r"(?m)^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();

    regex.captures_iter(input)
        .map(|claim| {
            let id = to_u32(&claim[1]);
            let coords = (to_u32(&claim[2]), to_u32(&claim[3]));
            let size = (to_u32(&claim[4]), to_u32(&claim[5]));

            Claim {
                id,
                coords,
                size
            }
        })
        .collect()
}

fn to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn calculate_overlapping_area(claims: &Vec<Claim>) -> u32 {
    let grid = build_access_grid(claims);

    let mut overlapping_area = 0;

    for point in grid {
        if point > 1 {
            overlapping_area += 1;
        }
    }

    overlapping_area
}

fn get_intact_claim(claims: &Vec<Claim>) -> Option<&Claim> {
    let grid_size = get_grid_size(claims);
    let grid = build_access_grid(claims);

    for claim in claims {
        let mut intact = true;

        for x in 0..claim.size.0 {
            for y in 0..claim.size.1 {
                let pos = (claim.coords.0 + x) * grid_size.1 + (claim.coords.1 + y);
                let point = grid.get(pos as usize).unwrap();

                if *point > 1 {
                    intact = false;
                }
            }
        }

        if intact {
            return Some(&claim);
        }
    }

    return None;
}

fn get_grid_size(claims: &Vec<Claim>) -> (u32, u32) {
    let mut grid_size = (0u32, 0u32);
    
    for claim in claims {
        if claim.coords.0 + claim.size.0 > grid_size.0 {
            grid_size.0 = claim.coords.0 + claim.size.0;
        }

        if claim.coords.1 + claim.size.1 > grid_size.1 {
            grid_size.1 = claim.coords.1 + claim.size.1;
        }
    }

    grid_size
}

fn build_access_grid(claims: &Vec<Claim>) -> Vec<u32> {
    let grid_size = get_grid_size(&claims);

    let mut grid: Vec<u32> = vec![0; (grid_size.0 * grid_size.1) as usize];

    for claim in claims {
        for x in 0..claim.size.0 {
            for y in 0..claim.size.1 {
                let pos = (claim.coords.0 + x) * grid_size.1 + (claim.coords.1 + y);
                let point = grid.get_mut(pos as usize).unwrap();
                *point += 1;
            }
        }
    }

    grid
}
