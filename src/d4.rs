//! A solution to day 4 year 2024.
//! https://adventofcode.com/2024/day/4

use regex::Regex;

type Model = Puzzle;
type Answer = u32;

pub struct Puzzle {
    input_matrix: Vec<Vec<char>>,
    horizantal: Vec<String>,
    vertical: Vec<String>,
    diagnal_right: Vec<String>,
    diagnal_left: Vec<String>,
}

impl Puzzle {
    pub fn new(input_matrix: Vec<Vec<char>>) -> Puzzle {
        let mut horizantal: Vec<String> = vec![];
        let mut vertical: Vec<String> = vec![];
        let mut diagnal_right: Vec<String> = vec![];
        let mut diagnal_left: Vec<String> = vec![];

        let num_rows = input_matrix.len();
        let row_len = input_matrix[0].len();

        // generate horizantal rows
        for row in &input_matrix {
            horizantal.push(String::from_iter(row));
        }

        // generate vertical rows
        for j in 0..row_len {
            let mut vert_row = String::new();
            for row in &input_matrix {
                vert_row.push(row[j]);
            }
            vertical.push(vert_row);
        }

        // generate diagnals

        // initialize the diagnal rows
        let num_diag_rows = (num_rows * 2) - 1;
        for i in 0..num_diag_rows {
            diagnal_right.push(String::new());
            diagnal_left.push(String::new());
        }

        // transpose diagnals to right leaning rows
        for i in 0..num_rows {
            for j in 0..row_len {
                let c = input_matrix[i][j];
                diagnal_right[i + j].push(c);
            }
        }

        // transpose diagnals to left leaning rows
        let mut input_matrix_reversed = input_matrix.clone();
        input_matrix_reversed.reverse();
        for i in 0..num_rows {
            for j in 0..row_len {
                let c = input_matrix_reversed[i][j];
                diagnal_left[i + j].push(c);
            }
        }

        Puzzle {
            input_matrix,
            horizantal,
            vertical,
            diagnal_right,
            diagnal_left,
        }
    }

    pub fn find_xmas(rows: &Vec<String>) -> u32 {
        let mut xmas_count: u32 = 0;

        let re = Regex::new(r"XMAS").unwrap();

        for row in rows {
            // Find all matches
            for mat in re.find_iter(row) {
                xmas_count += 1;
            }

            let reverse: String = row.chars().rev().collect();

            for mat in re.find_iter(&reverse) {
                xmas_count += 1;
            }
        }

        xmas_count
    }

    pub fn find_all(&self) -> u32 {
        let total_horizantal = Puzzle::find_xmas(&self.horizantal);
        let total_vertical = Puzzle::find_xmas(&self.vertical);
        let total_diagnal_right = Puzzle::find_xmas(&self.diagnal_right);
        let total_diagnal_left = Puzzle::find_xmas(&self.diagnal_left);

        // println!("h: {total_horizantal}");
        // println!("v: {total_vertical}");
        // println!("d right: {total_diagnal_right}");
        // println!("d left: {total_diagnal_left}");

        total_horizantal + total_vertical + total_diagnal_right + total_diagnal_left
    }
}

pub fn parse(input: String) -> Model {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut input_matrix: Vec<Vec<char>> = vec![];

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        input_matrix.push(chars);
    }

    Model::new(input_matrix)
}

pub fn part1(model: Model) -> Answer {
    model.find_all()
}

pub fn part2(model: Model) -> Answer {
    let mut total_x_mas = 0;

    for i in 1..&model.input_matrix.len() - 1 {
        let row = &model.input_matrix[i];
        for j in 1..row.len() - 1 {
            if model.input_matrix[i][j] == 'A' {
                // check corners for MS
                let upper_left = model.input_matrix[i - 1][j - 1];
                let upper_right = model.input_matrix[i - 1][j + 1];
                let lower_left = model.input_matrix[i + 1][j - 1];
                let lower_right = model.input_matrix[i + 1][j + 1];

                if ((upper_left == 'M' && lower_right == 'S')
                    || (upper_left == 'S' && lower_right == 'M'))
                    && ((upper_right == 'M' && lower_left == 'S')
                        || (upper_right == 'S' && lower_left == 'M'))
                {
                    total_x_mas += 1;
                }
            }
        }
    }
    total_x_mas
}
