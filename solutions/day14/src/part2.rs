use std::collections::HashMap;

use crate::utils::{apply_substitutions, get_freq_diff, get_pairs};

pub fn solution((template, rules): &(String, HashMap<(char, char), char>)) -> i64 {
    let mut pair_map = get_pairs(template);

    for _i in 0..40 {
        pair_map = apply_substitutions(pair_map, rules);
    }

    get_freq_diff(template, pair_map) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(2188189693529, solution(&crate::utils::get_input("test")));
    }
}
