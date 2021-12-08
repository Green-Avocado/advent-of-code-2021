use std::{
    fs::File,
    io::{BufReader, Lines},
    collections::VecDeque,
};

pub fn solution(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut ans = 0;
    let mut group = VecDeque::with_capacity(3);

    for _i in 0..3 {
        let depth = lines.next().unwrap().unwrap().parse().unwrap();
        group.push_front(depth);
    }

    for line in lines {
        if let Ok(s) = line {
            let depth: u32 = s.parse().unwrap();
            let old_depth = group.pop_back().unwrap();
            group.push_front(depth);

            if old_depth < depth {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(5, solution(crate::utils::get_input("test")));
    }
}
