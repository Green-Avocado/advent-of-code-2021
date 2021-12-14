use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> (Vec<(u16, u16)>, Vec<(u16, bool)>) {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");
    let mut points = Vec::new();
    let mut folds = Vec::new();
    
    for line in BufReader::new(input).lines() {
        if let Ok(s) = line {
        }
    }

    (points, folds)
}
