use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> Vec<u32> {
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
