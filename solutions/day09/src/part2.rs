use std::{
    fs::File,
    io::{BufReader, Lines},
};

use crate::utils::{get_line_basins, line_to_vec};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut all_basins = Vec::new();
    let mut prev_basins = None;

    for line in lines {
        if let Ok(s) = line {
            prev_basins = Some(get_line_basins(&line_to_vec(s), prev_basins, &mut all_basins));
        }
    }

    let mut greatest_basins = [0; 3];

    for basin in all_basins {
        let b = basin.borrow();
        if b.parent.is_none() {
            if b.size > greatest_basins[0] {
                greatest_basins[0] = b.size;
                greatest_basins.sort_unstable();
            }
        }
    }

    let mut ans = 1;

    for i in greatest_basins {
        ans *= i as i64;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(1134, solution(crate::utils::get_input("test")));
    }
}
