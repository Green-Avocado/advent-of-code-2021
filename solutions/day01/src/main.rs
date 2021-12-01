mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    BufReader::new(input).lines()
}

fn main() {
    println!("Part 1: {}", part1::solution(get_input("input")));
    println!("Part 2: {}", part2::solution(get_input("input")));
}
