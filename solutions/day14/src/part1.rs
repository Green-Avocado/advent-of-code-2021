use std::collections::HashMap;

pub fn solution((template, rules): &(String, HashMap<(char, char), char>)) -> i64 {
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
