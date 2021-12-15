use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> (String, Vec<(char, char, char)>) {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");
    let mut lines = BufReader::new(input).lines();

    let template = lines.next().unwrap().unwrap();
    let mut rules = Vec::new();

    for line in lines {
        if let Ok(s) = line {
            if let Some(split) = s.split_once(" -> ") {
                let mut left_chars = split.0.chars();

                let c1 = left_chars.next().unwrap();
                let c2 = left_chars.next().unwrap();
                let c3 = split.1.chars().next().unwrap();
                
                rules.push((c1, c2, c3));
            }
        }
    }

    (template, rules)
}
