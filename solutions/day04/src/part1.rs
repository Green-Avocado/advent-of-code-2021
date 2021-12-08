use crate::utils;

use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let (nums, mut boards) = utils::parse_input(lines);

    for num in &nums {
        for board in &mut boards {
            if let Some(x) = board.check_num(*num) {
                return x;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(4512, solution(crate::utils::get_input("test")));
    }
}
