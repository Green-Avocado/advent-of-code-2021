use std:: collections::HashMap;

pub fn solution(graph: &HashMap<String, Vec<String>>) -> i64 {
    count_paths(graph, &"start".to_string(), Vec::new(), true)
}

fn count_paths(graph: &HashMap<String, Vec<String>>, node: &String, mut path: Vec<String>, extra_visit: bool) -> i64 {
    if node == "end" {
        return 1;
    }

    let mut count = 0;
    path.push(node.clone());

    let branches = graph.get(node).unwrap();

    for branch in branches {
        if branch == "start" {
            continue;
        }

        let condition = branch.to_uppercase() == *branch || !path.contains(branch);
        if condition || extra_visit {
            count += count_paths(graph, branch, path.clone(), condition && extra_visit);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample0() {
        assert_eq!(36, solution(&crate::utils::get_input("test0")));
    }

    #[test]
    fn sample1() {
        assert_eq!(103, solution(&crate::utils::get_input("test1")));
    }

    #[test]
    fn sample2() {
        assert_eq!(3509, solution(&crate::utils::get_input("test2")));
    }
}
