//! A solution to day 2 year 2024.
//! https://adventofcode.com/2024/day/2

use std::cmp::Ordering;

type Model = Vec<Report>;
type Answer = u32;

#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}

const DELTA_THRESHOLD: u8 = 3;

#[derive(Debug)]
pub struct Report {
    pub levels: Vec<u8>,
}

impl From<&str> for Report {
    fn from(s: &str) -> Self {
        let levels = s
            .split_whitespace()
            .filter_map(|num| num.parse::<u8>().ok())
            .collect();

        Report { levels }
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let first = self.levels[0];
        let second = self.levels[1];

        let direction = match first.cmp(&second) {
            Ordering::Greater => Direction::Desc,
            Ordering::Less => Direction::Asc,
            Ordering::Equal => return false,
        };

        let mut iter = self.levels.iter().peekable();

        while let Some(current) = iter.next() {
            if let Some(next) = iter.peek() {
                // check the delta is within threshold
                if current.abs_diff(**next) > DELTA_THRESHOLD {
                    return false;
                }

                match direction {
                    Direction::Asc => {
                        // must be accending
                        if current >= next {
                            return false;
                        }
                    }
                    Direction::Desc => {
                        // must be descending
                        if current <= next {
                            return false;
                        }
                    }
                }
            }
        }

        dbg!(direction);

        true
    }
}

pub fn parse(input: String) -> Model {
    let mut reports: Vec<Report> = vec![];

    // read each line and parse each report
    let lines: Vec<&str> = input.trim().split("\n").collect();

    for line in lines {
        reports.push(Report::from(line));
    }

    reports
}

pub fn part1(model: Model) -> Answer {
    let mut total_safe: u32 = 0;

    model.iter().for_each(|r| {
        println!("{:?}", r);

        if r.is_safe() {
            total_safe += 1;
        }
    });

    total_safe
}

pub fn part2(model: Model) -> Answer {
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d2");
//     const EXAMPLE: &str = include_str!("../examples/d2");
//
//     // #[test]
//     // fn d2p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         "put part 1 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d2p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d2p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d2p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
