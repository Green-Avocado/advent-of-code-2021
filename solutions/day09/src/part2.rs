use std::{
    fs::File,
    io::{BufReader, Lines},
};

use crate::utils::{get_line_basins, line_to_vec};

pub fn solution(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;

    let mut all_basins = Vec::new();
    let mut prev_basins = None;

    for line in lines {
        if let Ok(s) = line {
            prev_basins = Some(get_line_basins(&line_to_vec(s), prev_basins, &mut all_basins));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(0, solution(crate::utils::get_input("test")));
    }
}
