use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> (Vec<(u16, u16)>, Vec<(u16, bool)>) {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");
    let mut lines = BufReader::new(input).lines();
    let mut points = Vec::new();
    let mut folds = Vec::new();
    
    while let Some(Ok(s)) = lines.next() {
        if let Some((a, b)) = s.split_once(',') {
            points.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            break;
        }
    }

    while let Some(Ok(s)) = lines.next() {
        if let Some(fold) = s.strip_prefix("fold along ") {
            let (a, b) = fold.split_once('=').unwrap();

            let horizontal = a == "y";
            let location = b.parse().unwrap();

            folds.push((location, horizontal));
        } else {
            break;
        }
    }

    (points, folds)
}
