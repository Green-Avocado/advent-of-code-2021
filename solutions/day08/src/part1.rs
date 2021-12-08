use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;

    for line in lines {
        if let Ok(s) = line {
            s.split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .for_each(|x| match x.len() {
                    2 | 3 | 4 | 7 => ans += 1,
                    _ => (),
                })
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(26, solution(crate::utils::get_input("test")));
    }
}
