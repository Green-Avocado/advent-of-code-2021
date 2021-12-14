use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

pub fn get_input(filename: &str) -> HashMap<String, Vec<String>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");
    let mut graph = HashMap::new();

    for line in BufReader::new(input).lines() {
        if let Ok(s) = line {
            let split = s.split_once('-').unwrap();
            let (node_a, node_b) = (split.0.to_string(), split.1.to_string());

            graph.entry(node_a.clone()).or_insert(Vec::new()).push(node_b.clone());
            graph.entry(node_b.clone()).or_insert(Vec::new()).push(node_a.clone());
        }
    }

    graph
}
