use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;

    for line in lines {
        if let Ok(s) = line {
            let (pattern_string, output_string) = s.split_once(" | ").unwrap();
            let mut one_map = 0;
            let mut four_map = 0;
            let eight_map = 0b01111111;

            for ps in pattern_string.split_ascii_whitespace() {
                let bitmap = to_bitmap(ps);
                match count_bits(bitmap) {
                    2 => one_map = bitmap,
                    4 => four_map = bitmap,
                    _ => (),
                }
            }

            let mut display = 0;
            for os in output_string.split_ascii_whitespace() {
                display *= 10;
                display += match os.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 => {
                        let bitmap = to_bitmap(os);
                        if bitmap | one_map == bitmap {
                            3
                        } else if bitmap | four_map == eight_map {
                            2
                        } else {
                            5
                        }
                    }
                    6 => {
                        let bitmap = to_bitmap(os);
                        if bitmap | four_map == bitmap {
                            9
                        } else if bitmap | one_map == bitmap {
                            0
                        } else {
                            6
                        }
                    }
                    7 => 8,
                    _ => unreachable!(),
                };
            }

            ans += display;
        }
    }

    ans
}

fn to_bitmap(s: &str) -> u8 {
    let mut bitmap = 0b0;

    for c in s.chars() {
        bitmap += 0b1 << c as u8 - 'a' as u8;
    }

    bitmap
}

fn count_bits(num: u8) -> u8 {
    let mut ones = 0;

    for i in 0..8 {
        if num & (1 << i) > 0 {
            ones += 1;
        }
    }

    ones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(61229, solution(crate::utils::get_input("test")));
    }

    #[test]
    fn test_to_bitmap() {
        assert_eq!(0b00000000, to_bitmap(""));
        assert_eq!(0b00000001, to_bitmap("a"));
        assert_eq!(0b01000000, to_bitmap("g"));
        assert_eq!(0b01010101, to_bitmap("aceg"));
        assert_eq!(0b01111111, to_bitmap("abcdefg"));
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(0, count_bits(0b00000000));
        assert_eq!(1, count_bits(0b00000010));
        assert_eq!(4, count_bits(0b10101010));
        assert_eq!(8, count_bits(0b11111111));
    }
}
