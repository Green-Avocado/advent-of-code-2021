use std::collections::HashSet;

pub fn solution((points, folds): &(HashSet<(u16, u16)>, Vec<(u16, bool)>)) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(0, solution(&crate::utils::get_input("test")));
    }
}
