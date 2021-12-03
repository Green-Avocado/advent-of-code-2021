use std::{
    fs::File,
    io::{BufReader, Lines}, collections::LinkedList, str::Chars,
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut oxygen_list = LinkedList::new();
    let mut carbon_list = LinkedList::new();

    let mut strings = Vec::new();

    for line in lines {
        if let Ok(s) = line {
            strings.push(s);
        }
    }

    for string in &strings {
        let digits = string.chars();
        oxygen_list.push_front(digits.clone());
        carbon_list.push_front(digits.clone());
    }

    (get_rating(true, oxygen_list) * get_rating(false, carbon_list)) as i64
}

fn get_rating(most_common: bool, mut list: LinkedList<Chars>) -> u32 {
    let mut rating = 0;

    loop {
        let mut ones = LinkedList::new();
        let mut zeros = LinkedList::new();

        for mut digits in list {
            match digits.next() {
                Some('0') => {
                    zeros.push_front(digits.clone());
                }
                Some('1') => {
                    ones.push_front(digits.clone());
                }
                _ => break,
            }
        }

        rating *= 2;

        let condition = if most_common {
            ones.len() >= zeros.len()
        } else {
            ones.len() < zeros.len()
        };

        list = if condition {
            rating += 1;
            ones
        } else {
            zeros
        };

        if list.len() == 1 {
            for x in list.pop_front().unwrap() {
                rating *= 2;
                if x == '1' {
                    rating += 1;
                }
            }

            break;
        }
    }

    rating
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(230, solution(crate::get_input("test")));
    }
}
