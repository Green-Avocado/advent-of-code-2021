use crate::utils;

use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let (nums, mut boards) = utils::parse_input(lines);
    let mut last_score = 0;

    for num in &nums {
        let mut i = 0;
        let mut to_remove = Vec::new();

        for board in &mut boards {
            if let Some(x) = board.check_num(*num) {
                last_score = x;
                to_remove.push(i);
            }
            i += 1;
        }

        for index in to_remove.iter().rev() {
            boards.remove(*index);
        }
    }

    last_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(1924, solution(crate::utils::get_input("test")));
    }
}
