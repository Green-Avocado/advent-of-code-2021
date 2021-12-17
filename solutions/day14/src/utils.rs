use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

pub fn get_input(filename: &str) -> (String, HashMap<(char, char), char>) {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");
    let mut lines = BufReader::new(input).lines();

    let template = lines.next().unwrap().unwrap();
    let mut rules = HashMap::new();

    for line in lines {
        if let Ok(s) = line {
            if let Some(split) = s.split_once(" -> ") {
                let mut left_chars = split.0.chars();

                let c1 = left_chars.next().unwrap();
                let c2 = left_chars.next().unwrap();
                let c3 = split.1.chars().next().unwrap();
                
                rules.insert((c1, c2), c3);
            }
        }
    }

    (template, rules)
}

pub fn apply_substitutions(string: String, rules: &HashMap<(char, char), char>) -> String {
    let mut result = String::new();

    let mut chars = string.chars();

    let mut right = chars.next().unwrap();

    loop {
        result.push(right);

        let left = right;

        if let Some(next_char) = chars.next() {
            right = next_char;

            if let Some(insert_char) = rules.get(&(left, right)) {
                result.push(*insert_char);
            }
        } else {
            break;
        }
    }

    result
}
