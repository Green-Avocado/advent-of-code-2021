use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    for line in lines {
        if let Ok(s) = line {
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(0, solution(crate::utils::get_input("test0")));
        assert_eq!(0, solution(crate::utils::get_input("test1")));
        assert_eq!(0, solution(crate::utils::get_input("test2")));
    }
}
