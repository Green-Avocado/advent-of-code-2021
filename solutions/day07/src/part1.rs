pub fn solution(frequencies: &Vec<u32>) -> i64 {
    let mut iterator = frequencies.iter();

    let mut left = *iterator.next().unwrap();
    let mut right = *iterator.next_back().unwrap();

    let mut total = 0;

    loop {
        if left > right {
            total += right;

            if let Some(x) = iterator.next_back() {
                right += *x;
            } else {
                break;
            }
        } else {
            total += left;

            if let Some(x) = iterator.next() {
                left += *x;
            } else {
                break;
            }
        }
    }

    total as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(37, solution(&crate::get_input("test")));
    }
}
