use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut scores = Vec::new();

    'lines_loop: for line in lines {
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
                            continue 'lines_loop;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let mut score = 0;

            for c in stack.iter().rev() {
                score *= 5;
                score += match *c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                }
            }

            scores.push(score);
        }
    }

    scores.sort();

    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(288957, solution(crate::utils::get_input("test")));
    }
}
