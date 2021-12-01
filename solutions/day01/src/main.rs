use std::{
    fs::File,
    io::{BufReader, BufRead, Lines},
};

fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let input = File::open(format!("./data/{}", filename))
        .expect("Could not read file");

    BufReader::new(input).lines()
}

fn solution(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;
    let mut prev_depth = lines.next().unwrap().unwrap().parse().unwrap();

    for line in lines {
        if let Ok(s) = line {
            let depth: u32 = s.parse().unwrap();

            if depth > prev_depth {
                ans += 1;
            }

            prev_depth = depth;
        }
    }

    ans
}

fn main() {
    let lines = get_input("input");
    let ans = solution(lines);

    println!("Increases: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(7, solution(get_input("test")));
    }
}