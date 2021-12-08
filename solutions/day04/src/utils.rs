use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let input = File::open(format!("./data/{}", filename)).expect("Could not read file");

    BufReader::new(input).lines()
}

pub struct BingoBoard {
    pub points_remaining: u32,
    numbers: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl BingoBoard {
    #[inline]
    pub fn new() -> Self {
        BingoBoard {
            points_remaining: 0,
            numbers: [[0; 5]; 5],
            marked: [[false; 5]; 5],
        }
    }

    #[inline]
    pub fn insert(&mut self, num: u32, row: usize, col: usize) {
        self.numbers[row][col] = num;
        self.points_remaining += num;
    }

    #[inline]
    pub fn check_num(&mut self, num: u32) -> Option<i64> {
        let mut row = 0;
        for nums in self.numbers {
            let mut col = 0;
            for board_num in nums {
                if board_num == num {
                    self.marked[row][col] = true;
                    self.points_remaining -= num;

                    if self.check_win(row, col) {
                        return Some(self.points_remaining as i64 * num as i64);
                    }

                    break;
                }
                col += 1;
            }
            row += 1;
        }

        None
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        let mut col_win = true;
        let mut row_win = true;

        for i in 0..5 {
            if !self.marked[row][i] {
                row_win = false;
            }

            if !self.marked[i][col] {
                col_win = false;
            }
        }

        row_win || col_win
    }
}

pub fn parse_input(mut lines: Lines<BufReader<File>>) -> (Vec<u32>, Vec<BingoBoard>) {
    let nums_string = lines.next().unwrap().unwrap();
    let mut nums = Vec::new();

    for num_str in nums_string.split(',') {
        let num: u32 = num_str.parse().unwrap();
        nums.push(num);
    }

    let mut row = 0;
    let mut boards = Vec::new();
    let mut current_board = BingoBoard::new();

    for line in lines {
        if let Ok(s) = line {
            if s.len() == 0 {
                row = 0;
            } else {
                let mut col = 0;

                for num_str in s.split_ascii_whitespace() {
                    let num: u32 = num_str.parse().unwrap();
                    current_board.insert(num, row, col);
                    col += 1;
                }

                if row == 4 {
                    boards.push(current_board);
                    current_board = BingoBoard::new();
                }

                row += 1;
            }
        }
    }

    (nums, boards)
}
