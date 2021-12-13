use crate::utils;

const STEPS: u8 = 100;

pub fn solution(grid: &Vec<Vec<u8>>) -> i64 {
    let mut octopi = grid.clone();
    let mut flashes = 0;

    for _i in 0..STEPS {
        utils::grid_increment_all(&mut octopi);
        flashes += utils::grid_flash_all(&mut octopi) as i64;
        utils::grid_reset_flashes(&mut octopi);
    }

    flashes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(1656, solution(&crate::utils::get_input("test")));
    }
}
