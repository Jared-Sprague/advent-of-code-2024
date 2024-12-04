//! A solution to day 3 year 2024.
//! https://adventofcode.com/2024/day/3

use anyhow::{anyhow, Result};
use regex::Regex;

type Model = PartsModel;
type Answer = i64;

enum State {
    Enabled,
    Disabled,
}

pub struct PartsModel {
    part_1: Vec<Mul>,
    part_2: Vec<Mul>,
}

pub struct Mul {
    x: Option<i64>,
    y: Option<i64>,
    mul_x: Option<Box<Mul>>,
    mul_y: Option<Box<Mul>>,
}

impl Mul {
    pub fn multiply(&self) -> anyhow::Result<i64> {
        if self.x.is_some() && self.y.is_some() {
            return Ok(self.x.unwrap() * self.y.unwrap());
        } else if self.x.is_some() && self.mul_y.is_some() {
            let mul_y = self.mul_y.as_ref().unwrap();
            if let Ok(y) = mul_y.multiply() {
                return Ok(self.x.unwrap() * y);
            } else {
                return Err(anyhow!("invalid mul_y result"));
            }
        } else if self.mul_x.is_some() && self.y.is_some() {
            let mul_x = self.mul_x.as_ref().unwrap();
            if let Ok(x) = mul_x.multiply() {
                return Ok(x * self.y.unwrap());
            } else {
                return Err(anyhow!("invalid mul_x result"));
            }
        }

        Err(anyhow!("invalid inputs"))
    }
}

pub fn get_muls(input: &str) -> Vec<Mul> {
    let mut muls: Vec<Mul> = vec![];

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        muls.push(Mul {
            x: Some(x.parse::<i64>().unwrap()),
            y: Some(y.parse::<i64>().unwrap()),
            mul_x: None,
            mul_y: None,
        });
    }

    muls
}

pub fn parse(input: String) -> Model {
    let mut part_1: Vec<Mul> = vec![];
    let mut part_2: Vec<Mul> = vec![];

    let input = input.trim();

    part_1 = get_muls(input);

    let mut enabled_char_buf = String::new();
    let mut disabled_char_buf = String::new();

    let mut state = State::Enabled;

    // parse part 2
    for char in input.chars() {
        match state {
            State::Enabled => {
                enabled_char_buf.push(char);

                if enabled_char_buf.ends_with("don't()") {
                    state = State::Disabled;
                    enabled_char_buf = enabled_char_buf.trim_end_matches("don't()").to_string()
                }
            }
            State::Disabled => {
                disabled_char_buf.push(char);

                if disabled_char_buf.ends_with("do()") {
                    state = State::Enabled;
                    disabled_char_buf = "".to_string();
                }
            }
        }
    }

    part_2 = get_muls(&enabled_char_buf);

    PartsModel { part_1, part_2 }
}

pub fn part1(model: Model) -> Answer {
    let mut total: i64 = 0;

    for mul in model.part_1 {
        total += mul.multiply().unwrap();
    }

    total
}

pub fn part2(model: Model) -> Answer {
    let mut total: i64 = 0;

    for mul in model.part_2 {
        total += mul.multiply().unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_multiply() -> Result<()> {
        // Test case 1: Both x and y are provided
        let mul1 = Mul {
            x: Some(3),
            y: Some(4),
            mul_x: None,
            mul_y: None,
        };
        assert_eq!(mul1.multiply()?, 12);

        // Test case 2: x is provided, and mul_y calculates the value
        let mul2 = Mul {
            x: Some(2),
            y: None,
            mul_x: None,
            mul_y: Some(Box::new(Mul {
                x: Some(3),
                y: Some(5),
                mul_x: None,
                mul_y: None,
            })),
        };
        assert_eq!(mul2.multiply()?, 30);

        // Test case 3: y is provided, and mul_x calculates the value
        let mul3 = Mul {
            x: None,
            y: Some(6),
            mul_x: Some(Box::new(Mul {
                x: Some(2),
                y: Some(7),
                mul_x: None,
                mul_y: None,
            })),
            mul_y: None,
        };
        assert_eq!(mul3.multiply()?, 84);

        // Test case 4: Neither x nor y is provided
        let mul4 = Mul {
            x: None,
            y: None,
            mul_x: None,
            mul_y: None,
        };
        assert!(mul4.multiply().is_err());

        // Test case 5: Nested computation with both mul_x and mul_y
        let mul5 = Mul {
            x: None,
            y: None,
            mul_x: Some(Box::new(Mul {
                x: Some(2),
                y: Some(3),
                mul_x: None,
                mul_y: None,
            })),
            mul_y: Some(Box::new(Mul {
                x: Some(4),
                y: Some(5),
                mul_x: None,
                mul_y: None,
            })),
        };
        assert!(mul5.multiply().is_err(), "mul5 should have invalid inputs");

        Ok(())
    }
}
