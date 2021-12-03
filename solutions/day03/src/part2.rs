use std::{
    fs::File,
    io::{BufReader, Lines},
    collections::VecDeque,
};

pub fn solution(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;

    for line in lines {
        if let Ok(s) = line {
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(5, solution(crate::get_input("test")));
    }
}
