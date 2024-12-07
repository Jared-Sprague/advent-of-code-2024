//! A solution to day 7 year 2024.
//! https://adventofcode.com/2024/day/7

use std::sync::atomic::{AtomicU64, Ordering};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

type Model = Vec<Calibration>;
type Answer = u64;

#[derive(Debug)]
pub struct Calibration {
    value: u64,
    operands: Vec<u32>,
}

impl From<String> for Calibration {
    fn from(input: String) -> Self {
        let parts: Vec<&str> = input.split(":").collect();
        let value = parts[0].trim().parse::<u64>().unwrap();
        let operands: Vec<&str> = parts[1].trim().split(" ").collect();
        let mut operands: Vec<u32> = operands
            .iter()
            .map(|o| o.trim().parse::<u32>().unwrap())
            .collect();

        // reverse to read right-to-left using pop() which is faster than remove(0)
        operands.reverse();

        Calibration { value, operands }
    }
}

impl Calibration {
    fn calculate_value(&self, operators: &mut Vec<char>) -> u64 {
        let mut value = 0u64;
        let mut operands = self.operands.clone();

        // reverse to read right-to-left using pop() which is faster than remove(0)
        operators.reverse();

        while !operands.is_empty() {
            let left_operand = if value == 0 {
                operands.pop().unwrap() as u64
            } else {
                value
            };
            let next_operator = operators.pop().unwrap();
            let right_operand = operands.pop().unwrap();

            value = match next_operator {
                '*' => left_operand * right_operand as u64,
                '+' => left_operand + right_operand as u64,
                '|' => {
                    let concat_value_string = format!("{left_operand}{right_operand}");
                    concat_value_string.parse::<u64>().unwrap()
                }
                _ => panic!("unknown operator"),
            };
        }

        value
    }
}

pub fn permute_with_two(size: usize) -> Vec<Vec<char>> {
    if size == 0 {
        return vec![vec![]];
    }

    let smaller_permutations = permute_with_two(size - 1);
    let mut result = Vec::new();

    for perm in smaller_permutations {
        let mut with_star = perm.clone();
        with_star.push('*');
        result.push(with_star);

        let mut with_plus = perm.clone();
        with_plus.push('+');
        result.push(with_plus);
    }

    result
}

pub fn permute_with_three(size: usize) -> Vec<Vec<char>> {
    if size == 0 {
        return vec![vec![]];
    }

    let smaller_permutations = permute_with_three(size - 1);
    let mut result = Vec::new();

    for perm in smaller_permutations {
        let mut with_star = perm.clone();
        with_star.push('*');
        result.push(with_star);

        let mut with_plus = perm.clone();
        with_plus.push('+');
        result.push(with_plus);

        let mut with_pipe = perm.clone();
        with_pipe.push('|');
        result.push(with_pipe);
    }

    result
}

pub fn parse(input: String) -> Model {
    let input = input.trim();
    let mut calibrations: Vec<Calibration> = vec![];

    let lines: Vec<&str> = input.split("\n").collect();
    for line in lines {
        let calibration = Calibration::from(line.to_string());
        calibrations.push(calibration);
    }

    calibrations
}

pub fn part1(model: Model) -> Answer {
    let mut sum = 0;
    let mut model = model;

    for calibration in model.iter_mut() {
        let operators_len = calibration.operands.len() - 1;
        let mut operators = permute_with_two(operators_len);

        // see if any of the operator permutations yields the correct value
        for ops in operators.iter_mut() {
            let value = calibration.calculate_value(ops);
            if calibration.value == value {
                // println!("correct: {} == {}", calibration.value, value);
                sum += value;
                break; // found a good combination, no need to keep going
            } else {
                // println!("incorrect: {} == {}", calibration.value, value);
            }
        }
    }

    sum
}

pub fn part2(model: Model) -> Answer {
    let sum = AtomicU64::new(0);
    let mut model = model;

    for calibration in model.iter_mut() {
        let operators_len = calibration.operands.len() - 1;
        let mut operators = permute_with_three(operators_len);

        let _ = operators.par_iter_mut().try_for_each(|o| {
            let value = calibration.calculate_value(o);
            if calibration.value == value {
                // println!("correct: {} == {}", calibration.value, value);
                sum.fetch_add(value, Ordering::Relaxed);
                Err(()) // found a good combination, break out of the loop
            } else {
                // println!("incorrect: {} == {}", calibration.value, value);
                Ok(()) // continue the loop
            }
        });
    }

    sum.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_from_string() {
        let input = "190: 10 19".to_string();
        let calibration: Calibration = input.into();

        assert_eq!(calibration.value, 190);
        assert_eq!(calibration.operands, vec![10, 19]);
    }

    #[test]
    fn test_permute_length_1() {
        let result = permute_with_two(1);
        let expected = vec![vec!['*'], vec!['+']];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_three_length_1() {
        let result = permute_with_three(1);
        let expected = vec![vec!['*'], vec!['+'], vec!['|']];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_length_2() {
        let result = permute_with_two(2);
        let expected = vec![
            vec!['*', '*'],
            vec!['*', '+'],
            vec!['+', '*'],
            vec!['+', '+'],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_with_three() {
        let result = permute_with_three(2);
        let expected = vec![
            vec!['*', '*'],
            vec!['*', '+'],
            vec!['*', '|'],
            vec!['+', '*'],
            vec!['+', '+'],
            vec!['+', '|'],
            vec!['|', '*'],
            vec!['|', '+'],
            vec!['|', '|'],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_length_3() {
        let result = permute_with_two(3);
        let expected = vec![
            vec!['*', '*', '*'],
            vec!['*', '*', '+'],
            vec!['*', '+', '*'],
            vec!['*', '+', '+'],
            vec!['+', '*', '*'],
            vec!['+', '*', '+'],
            vec!['+', '+', '*'],
            vec!['+', '+', '+'],
        ];
        assert_eq!(result, expected);
    }
}
