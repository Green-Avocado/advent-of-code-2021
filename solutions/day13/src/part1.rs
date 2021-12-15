use std::collections::HashSet;

pub fn solution((points, folds): &(HashSet<(u16, u16)>, Vec<(u16, bool)>)) -> i64 {
    let fold = folds.get(0).unwrap();

    crate::utils::fold_paper(points, fold).len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(17, solution(&crate::utils::get_input("test")));
    }
}
