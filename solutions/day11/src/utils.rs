use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input(filename: &str) -> Vec<Vec<u8>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    let mut grid = Vec::new();

    for line in BufReader::new(input).lines() {
        if let Ok(s) = line {
            let mut row = Vec::new();

            for c in s.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }

            grid.push(row);
        }
    }

    grid
}

pub fn grid_increment_all(grid: &mut Vec<Vec<u8>>) {
    for row in grid {
        for col in row {
            *col += 1;
        }
    }
}

pub fn grid_flash_all(grid: &mut Vec<Vec<u8>>) -> u8{
    let mut flashes = 0;

    let mut row_num = 0;

    'row_loop: loop {
        let mut col_num = 0;

        'col_loop: loop {
            if let Some(x) = grid.get(row_num) {
                if let Some(y) = x.get(col_num) {
                    if *y == 10 {
                        flashes += grid_flash(grid, row_num, col_num);
                    }

                    col_num += 1;
                } else {
                    break 'col_loop;
                }
            } else {
                break 'row_loop;
            }
        }

        row_num += 1;
    }

    flashes
}

fn grid_flash(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) -> u8 {
    let mut flashes = 1;

    let mut row_num = if row > 0 {
        row - 1
    } else {
        0
    };

    'row_loop: loop {
        if row_num > row + 1 {
            break;
        }

        let mut col_num = if col > 0 {
            col - 1
        } else {
            0
        };

        'col_loop: loop {
            if col_num > col + 1 {
                break;
            }

            if let Some(x) = grid.get_mut(row_num) {
                if let Some(y) = x.get_mut(col_num) {
                    if row_num == row && col_num == col {
                        *y = 11;
                    }

                    if *y < 10 {
                        *y += 1;

                        if *y == 10 {
                            flashes += grid_flash(grid, row_num, col_num);
                        }
                    }

                    col_num += 1;
                } else {
                    break 'col_loop;
                }
            } else {
                break 'row_loop;
            }
        }

        row_num += 1;
    }

    flashes
}

pub fn grid_reset_flashes(grid: &mut Vec<Vec<u8>>) {
    for x in grid {
        for y in x {
            if *y > 9 {
                *y = 0;
            }
        }
    }
}
