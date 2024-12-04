//! A solution to day 4 year 2024.
//! https://adventofcode.com/2024/day/4

use rayon::vec;

type Model = Puzzle;
type Answer = u32;

pub struct Puzzle {
    input_matrix: Vec<Vec<char>>,
    horizantal: String,
    vertical: String,
    diagnal: String,
}

impl Puzzle {
    pub fn new(input_matrix: Vec<Vec<char>>) -> Puzzle {
        let mut horizantal: String = String::new();
        let mut vertical: String = String::new();
        let mut diagnal: String = String::new();

        // generate horizantal string
        for row in &input_matrix {
            horizantal.extend(row);
        }

        Puzzle {
            input_matrix,
            horizantal,
            vertical,
            diagnal,
        }
    }

    pub fn find_horizantal(&self) -> u32 {
        let mut total_horizantal: u32 = 0;

        let re = Regex::new(r"XMAS|SAMX").unwrap(); // Create the regex pattern

        // Find all matches
        for mat in re.find_iter(text) {
            println!("Found match: {}", mat.as_str());
        }

        total_horizantal
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
    dbg!(model.horizantal);
    0
}

pub fn part2(model: Model) -> Answer {
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d4");
//     const EXAMPLE: &str = include_str!("../examples/d4");
//
//     // #[test]
//     // fn d4p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d4p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d4p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d4p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
