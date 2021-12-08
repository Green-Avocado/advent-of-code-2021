const TIMESPAN: u16 = 80;

pub fn solution(nums: &[u64; 9]) -> i64 {
    let mut nums = nums.clone();
    for _i in 0..TIMESPAN {
        nums.rotate_left(1);
        nums[6] += nums[8];
    }

    nums.iter().sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(5934, solution(&crate::utils::get_input("test")));
    }
}
