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

pub fn get_pairs(string: &String) -> HashMap<(char, char), u128> {
    let mut pair_map = HashMap::new();

    let mut chars = string.chars();

    let mut right = chars.next().unwrap();

    loop {
        let left = right;

        if let Some(next_char) =  chars.next() {
            right = next_char;

            *pair_map.entry((left, right)).or_insert(0) += 1;
        } else {
            break;
        }
    }

    pair_map
}

pub fn apply_substitutions(pair_map: HashMap<(char, char), u128>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), u128> {
    let mut new_map = HashMap::new();

    for ((left, right), count) in pair_map {
        if let Some(&insert_char) = rules.get(&(left, right)) {
            *new_map.entry((left, insert_char)).or_insert(0) += count;
            *new_map.entry((insert_char, right)).or_insert(0) += count;
        }
    }

    new_map
}

pub fn get_freq_diff(string: &String, pair_map: HashMap<(char, char), u128>) -> u128 {
    let mut freq_map = HashMap::new();

    for ((left, _right), count) in pair_map {
        *freq_map.entry(left).or_insert(0) += count;
    }

    *freq_map.entry(string.chars().last().unwrap()).or_insert(0) += 1;

    let mut low_freq = 0;
    let mut high_freq = 0;

    for &freq in freq_map.values() {
        if freq > high_freq || high_freq == 0 {
            high_freq = freq;
        }

        if freq < low_freq || low_freq == 0 {
            low_freq = freq;
        }
    }

    high_freq - low_freq
}
