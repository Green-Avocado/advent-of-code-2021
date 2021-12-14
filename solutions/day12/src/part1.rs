use std:: collections::HashMap;

pub fn solution(graph: &HashMap<String, Vec<String>>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(10, solution(&crate::utils::get_input("test0")));
        assert_eq!(19, solution(&crate::utils::get_input("test1")));
        assert_eq!(226, solution(&crate::utils::get_input("test2")));
    }
}
