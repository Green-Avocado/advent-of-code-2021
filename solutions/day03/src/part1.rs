use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut length = 0;
    let mut totals = Vec::new();

    for line in lines {
        if let Ok(s) = line {
            let mut i = 0;

            for c in s.chars() {
                let n = if c == '1' {
                    1
                } else {
                    0
                };

                match totals.get_mut(i) {
                    Some(x) => *x += n,
                    None => totals.push(n),
                }
                i += 1;
            }
        }

        length += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut multiplier = 1;

    for n in totals.iter().rev() {
        if *n * 2 > length {
            gamma += multiplier;
        } else {
            epsilon += multiplier;
        }
        
        multiplier *= 2;
    }

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(198, solution(crate::get_input("test")));
    }
}
