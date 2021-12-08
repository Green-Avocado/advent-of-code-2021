use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, Lines},
};

pub fn solution(lines: Lines<BufReader<File>>) -> i64 {
    let mut oxygen_list = VecDeque::new();
    let mut carbon_list = VecDeque::new();

    let mut nums = Vec::new();

    for line in lines {
        if let Ok(s) = line {
            nums.push(s.chars().map(|x| x == '1').collect::<Vec<bool>>());
        }
    }

    for num in &nums {
        oxygen_list.push_front(num.iter());
        carbon_list.push_front(num.iter());
    }

    (get_rating(true, oxygen_list) * get_rating(false, carbon_list)) as i64
}

fn get_rating(most_common: bool, mut list: VecDeque<std::slice::Iter<bool>>) -> u32 {
    let mut rating = 0;

    loop {
        let mut ones = VecDeque::new();
        let mut zeros = VecDeque::new();

        for mut digits in list {
            match digits.next() {
                Some(false) => {
                    zeros.push_front(digits.clone());
                }
                Some(true) => {
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
                if *x {
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
        assert_eq!(230, solution(crate::utils::get_input("test")));
    }
}
