use std::{
    fs::File,
    io::{BufReader, Lines},
};

use crate::utils::{get_local_mins, line_result_to_vec};

pub fn solution(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;

    let mut prev_nums = None;
    let mut next_nums = line_result_to_vec(lines.next());

    loop {
        if let Some(x) = next_nums {
            next_nums = line_result_to_vec(lines.next());

            get_local_mins(&x, prev_nums.as_ref(), next_nums.as_ref())
                .iter()
                .for_each(|n| ans += 1 + *n as i64);

            prev_nums = Some(x);
        } else {
            break;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn sample() {
        assert_eq!(15, solution(utils::get_input("test")));
    }
}
