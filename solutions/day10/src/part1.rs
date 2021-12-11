use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut points = 0;

    for line in lines {
        if let Ok(s) = line {
            let mut stack = Vec::new();

            for c in s.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(match c {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => unreachable!(),
                    }),
                    ')' | ']' | '}' | '>' => {
                        if c != stack.pop().unwrap() {
                            points += match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            };

                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(26397, solution(crate::utils::get_input("test")));
    }
}
