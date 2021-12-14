use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    BufReader::new(input).lines()
}
