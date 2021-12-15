use std::collections::HashSet;

pub fn solution((points, folds): &(HashSet<(u16, u16)>, Vec<(u16, bool)>)) -> String {
    let mut current_points = points.clone();

    for fold in folds {
        current_points = crate::utils::fold_paper(&current_points, fold);
    }

    let mut max_x = 0;
    let mut max_y = 0;

    for &(x, y) in &current_points {
        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }
    }

    let mut s = "\n".to_string();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if current_points.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }

        s.push('\n');
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!("\n\
        #####\n\
        #...#\n\
        #...#\n\
        #...#\n\
        #####\n\
        ", solution(&crate::utils::get_input("test")));
    }
}
