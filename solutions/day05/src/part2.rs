use crate::utils;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut points_map = HashMap::new();
    let mut overlaps = 0;

    for line in lines {
        if let Ok(s) = line {
            let (point0, point1) = utils::get_points(s);

            overlaps += utils::fill_points(point0, point1, &mut points_map) as i64;
        }
    }

    overlaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(12, solution(crate::utils::get_input("test")));
    }
}
