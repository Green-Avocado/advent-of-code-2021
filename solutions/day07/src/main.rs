mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(filename: &str) -> Vec<u16> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    BufReader::new(input)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let nums = get_input("input");

    println!("Part 1: {}", part1::solution(&nums));
    println!("Part 2: {}", part2::solution(&nums));
}
