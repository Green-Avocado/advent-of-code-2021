use crate::utils;

pub fn solution(grid: &Vec<Vec<u8>>) -> i64 {
    let mut octopi = grid.clone();
    let mut steps = 0;

    loop {
        steps += 1;

        utils::grid_increment_all(&mut octopi);

        if utils::grid_flash_all(&mut octopi) == 100 {
            break;
        };

        utils::grid_reset_flashes(&mut octopi);
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(195, solution(&utils::get_input("test")));
    }
}
