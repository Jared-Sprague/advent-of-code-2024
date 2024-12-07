//! A solution to day 7 year 2024.
//! https://adventofcode.com/2024/day/7

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
        let operands: Vec<u32> = operands
            .iter()
            .map(|o| o.trim().parse::<u32>().unwrap())
            .collect();

        Calibration { value, operands }
    }
}

impl Calibration {
    fn calculate_value(&self, operators: Vec<char>) -> u64 {
        let mut operands = self.operands.clone();
        let mut value = 0u64;
        let mut operators = operators.clone();

        while !operands.is_empty() {
            let left_operand = if value == 0 {
                operands.remove(0) as u64
            } else {
                value
            };
            let next_operator = operators.remove(0);
            let right_operand = operands.remove(0);

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

    for calibration in model {
        let operators_len = calibration.operands.len() - 1;
        let operators = permute_with_two(operators_len);

        // see if any of the operator permutations yields the correct value
        for ops in operators {
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
    let mut sum = 0;

    for calibration in model {
        let operators_len = calibration.operands.len() - 1;
        let operators = permute_with_three(operators_len);

        // see if any of the operator permutations yields the correct value
        for ops in operators {
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
