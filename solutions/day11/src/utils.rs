use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> Vec<Vec<u8>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    let mut grid = Vec::new();

    for line in BufReader::new(input).lines() {
        if let Ok(s) = line {
            let mut row = Vec::new();

            for c in s.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }

            grid.push(row);
        }
    }

    grid
}
