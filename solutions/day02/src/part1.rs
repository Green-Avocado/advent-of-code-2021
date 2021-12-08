use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut horizontal_pos = 0;
    let mut vertical_pos: i32 = 0;

    for line in lines {
        if let Ok(s) = line {
            let mut split = s.split(" ");
            let direction = split.next().unwrap();
            let distance: u32 = split.next().unwrap().parse().unwrap();

            match direction {
                "forward" => horizontal_pos += distance,
                "up" => vertical_pos -= distance as i32,
                "down" => vertical_pos += distance as i32,
                _ => unreachable!(),
            }
        }
    }

    horizontal_pos as i64 * vertical_pos as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(150, solution(crate::utils::get_input("test")));
    }
}
