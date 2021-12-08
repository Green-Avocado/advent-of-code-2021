use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> [u64; 9] {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    let nums_string = BufReader::new(input).lines().next().unwrap().unwrap();
    let mut nums = [0; 9];

    for num_str in nums_string.split(',') {
        let i: usize = num_str.parse().unwrap();
        nums[i] += 1;
    }

    nums
}
