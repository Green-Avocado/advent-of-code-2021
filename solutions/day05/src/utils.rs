use std::collections::HashMap;

pub fn fill_points((mut x, mut y): (u16, u16), (x1, y1): (u16, u16), points_map: &mut HashMap<(u16, u16), bool>) -> u32 {
    let mut overlaps = 0;

    loop {
        points_map.entry((x, y))
            .and_modify(|e| {
                if !*e {
                    overlaps += 1;
                    *e = true;
                }
            })
            .or_insert(false);
        
        if x == x1 && y == y1 {
            break;
        }

        if x > x1 {
            x -= 1;
        }

        if x < x1 {
            x += 1;
        }

        if y > y1 {
            y -= 1;
        }

        if y < y1 {
            y += 1;
        }
    }

    overlaps
}

pub fn get_points(points_str: String) -> ((u16, u16), (u16, u16)) {
    let (point_str0, point_str1) = points_str.split_once(" -> ").unwrap();
    let point0 = parse_point(point_str0);
    let point1 = parse_point(point_str1);

    (point0, point1)
}

fn parse_point(point_str: &str) -> (u16, u16) {
    let (x_str, y_str) = point_str.split_once(',').unwrap();
    (x_str.parse().unwrap(), y_str.parse().unwrap())
}
