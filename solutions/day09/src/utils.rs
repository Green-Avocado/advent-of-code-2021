use std::{
    fs::File,
    io::{BufRead, BufReader, Lines}, rc::Rc, cell::RefCell,
};

pub struct Basin {
    parent: Option<Rc<RefCell<Basin>>>,
    size: u16,
}

pub fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    BufReader::new(input).lines()
}

pub fn get_line_basins(current_nums: &Vec<u8>, prev_basins: Vec<Option<Rc<RefCell<Basin>>>>) -> Vec<Option<Rc<RefCell<Basin>>>> {
    let mut basins = Vec::new();
    let mut prev_basins_iter = prev_basins.iter();

    let mut current_basin = Rc::new(RefCell::new(Basin {
        parent: None,
        size: 0,
    }));

    for n in current_nums {
        let prev_basin_option = prev_basins_iter.next().unwrap();

        if *n != 9 {
            if current_basin.borrow().size == 0 {
                basins.push(Some(Rc::clone(&current_basin)));
            }

            {
                current_basin.borrow_mut().size += 1;
            }

            if let Some(prev_basin) = prev_basin_option.as_ref() {
                let mut parent_basin = Rc::clone(prev_basin);

                loop {
                    if let Some(parent) = &Rc::clone(&parent_basin).borrow().parent {
                        parent_basin = Rc::clone(parent);
                    } else {
                        break;
                    }
                }

                if !Rc::ptr_eq(&parent_basin, &current_basin) {
                    {
                        parent_basin.borrow_mut().parent = Some(Rc::clone(&current_basin));
                    }
                    {
                        current_basin.borrow_mut().size += parent_basin.borrow().size;
                    }
                }
            }
        } else {
            basins.push(None);

            current_basin = Rc::new(RefCell::new(Basin {
                parent: None,
                size: 0,
            }));
        }
    }

    basins
}

pub fn get_local_mins(current_nums: &Vec<u8>, prev_nums: Option<&Vec<u8>>, next_nums: Option<&Vec<u8>>) -> Vec<u8> {
    let mut v = Vec::new();

    let mut mid = current_nums.iter();

    let mut top = if let Some(v) = prev_nums {
        Some(v.iter())
    } else {
        None
    };

    let mut bot = if let Some(v) = next_nums {
        Some(v.iter())
    } else {
        None
    };

    let mut prev = None;
    let mut next = mid.next();

    loop {
        if let Some(x) = next {
            next = mid.next();

            let up = if let Some(i) = top.as_mut() {
                i.next()
            } else {
                None
            };

            let down = if let Some(i) = bot.as_mut() {
                i.next()
            } else {
                None
            };

            if check_low_point(x, up, down, prev, next) {
                v.push(*x);
            }

            prev = Some(x);
        } else {
            break;
        }
    }

    v
}

pub fn line_result_to_vec(o: Option<Result<String, std::io::Error>>) -> Option<Vec<u8>> {
    if let Some(r) = o {
        Some(line_to_vec(r.unwrap()))
    } else {
        None
    }
}

pub fn line_to_vec(s: String) -> Vec<u8> {
    let mut v = Vec::new();

    for c in s.chars() {
        v.push(c.to_digit(10).unwrap() as u8);
    }

    v
}

fn check_low_point(num: &u8, up: Option<&u8>, down: Option<&u8>, left: Option<&u8>, right: Option<&u8>) -> bool {
    for i in [up, down, left, right] {
        if let Some(x) = i {
            if !(*num < *x) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_mins() {
        assert!(Vec::<u8>::new().eq(&get_local_mins(&Vec::new(), Some(&Vec::new()), Some(&Vec::new()))));
        assert!(vec![0, 1, 2].eq(&get_local_mins(&vec![0, 2, 1, 3, 2], Some(&vec![1, 0, 2, 0, 3]), Some(&vec![3, 3, 3, 3, 3]))));
    }

    #[test]
    fn test_line_result_to_vec() {
        assert!(line_result_to_vec(None).is_none());

        assert!(Vec::<u8>::new().eq(&line_result_to_vec(Some(Ok("".to_string()))).unwrap()));

        assert!(vec![4, 2, 0, 1, 3, 3, 7].eq(&line_result_to_vec(Some(Ok("4201337".to_string()))).unwrap()));
        assert!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].eq(&line_result_to_vec(Some(Ok("0123456789".to_string()))).unwrap()));
    }

    #[test]
    fn test_line_to_vec() {
        assert!(Vec::<u8>::new().eq(&line_to_vec("".to_string())));

        assert!(vec![4, 2, 0, 1, 3, 3, 7].eq(&line_to_vec("4201337".to_string())));
        assert!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].eq(&line_to_vec("0123456789".to_string())));
    }

    #[test]
    fn test_check_low_point() {
        assert!(!check_low_point(&0, Some(&0), Some(&0), Some(&0), Some(&0)));

        assert!(!check_low_point(&0, Some(&1), Some(&1), Some(&1), Some(&0)));
        assert!(!check_low_point(&0, Some(&1), Some(&1), Some(&0), Some(&1)));
        assert!(!check_low_point(&0, Some(&1), Some(&0), Some(&1), Some(&1)));
        assert!(!check_low_point(&0, Some(&0), Some(&1), Some(&1), Some(&1)));

        assert!(!check_low_point(&0, None, None, None, Some(&0)));
        assert!(!check_low_point(&0, None, None, Some(&0), None));
        assert!(!check_low_point(&0, None, Some(&0), None, None));
        assert!(!check_low_point(&0, Some(&0), None, None, None));

        assert!(check_low_point(&0, Some(&1), Some(&1), Some(&1), Some(&1)));
        assert!(check_low_point(&0, None, None, None, None));
    }
}
