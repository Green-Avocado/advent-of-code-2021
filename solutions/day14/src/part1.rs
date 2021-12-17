use std::collections::HashMap;

pub fn solution((template, rules): &(String, HashMap<(char, char), char>)) -> i64 {
    let mut freq_map = HashMap::new();

    let mut polymer = template.clone();

    for _i in 0..10 {
        polymer = crate::utils::apply_substitutions(polymer, rules);
    }

    for c in polymer.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let mut low_freq = 0;
    let mut high_freq = 0;

    for &freq in freq_map.values() {
        if freq > high_freq || high_freq == 0 {
            high_freq = freq;
        }

        if freq < low_freq || low_freq == 0 {
            low_freq = freq;
        }
    }

    high_freq - low_freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(1588, solution(&crate::utils::get_input("test")));
    }
}
