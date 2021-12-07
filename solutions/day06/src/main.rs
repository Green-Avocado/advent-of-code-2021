mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(filename: &str) -> [u64; 9] {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    let nums_string = BufReader::new(input).lines().next().unwrap().unwrap();
    let mut nums = [0; 9];

    for num_str in nums_string.split(',') {
        let i: usize = num_str.parse().unwrap();
        nums[i] += 1;
    }

    nums
}

fn main() {
    let nums_part1 = get_input("input");
    let nums_part2 = nums_part1.clone();

    println!("Part 1: {}", part1::solution(nums_part1));
    println!("Part 2: {}", part2::solution(nums_part2));
}
