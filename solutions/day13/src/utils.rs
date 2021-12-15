use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
};

pub fn get_input(filename: &str) -> (HashSet<(u16, u16)>, Vec<(u16, bool)>) {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");
    let mut lines = BufReader::new(input).lines();
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    
    while let Some(Ok(s)) = lines.next() {
        if let Some((a, b)) = s.split_once(',') {
            points.insert((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            break;
        }
    }

    while let Some(Ok(s)) = lines.next() {
        if let Some(fold) = s.strip_prefix("fold along ") {
            let (a, b) = fold.split_once('=').unwrap();

            let horizontal = a == "x";
            let location = b.parse().unwrap();

            folds.push((location, horizontal));
        } else {
            break;
        }
    }

    (points, folds)
}

pub fn fold_paper(points: &HashSet<(u16, u16)>, fold: &(u16, bool)) -> HashSet<(u16, u16)> {
    let mut final_points = HashSet::new();
    let &(fold_line, horizontal) = fold;

    for &(mut x, mut y) in points {
        let to_reflect;

        if horizontal {
            to_reflect = &mut x;
        } else {
            to_reflect = &mut y;
        }

        if *to_reflect >= fold_line {
            *to_reflect = fold_line - (*to_reflect - fold_line);
        }

        println!("{} {}", x, y);
        final_points.insert((x, y));
    }

    final_points
}
