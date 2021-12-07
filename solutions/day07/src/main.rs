mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(filename: &str) -> Vec<u32> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    let mut frequencies = Vec::new();

    BufReader::new(input)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .for_each(|x| {
            let n: u32 = x.parse().unwrap();
                
            while n as usize >= frequencies.len() {
                frequencies.push(0);
            }

            *frequencies.get_mut(n as usize).unwrap() += 1;
        });

    frequencies
}

fn main() {
    let frequencies = get_input("input");

    println!("Part 1: {}", part1::solution(&frequencies));
    println!("Part 2: {}", part2::solution(&frequencies));
}
