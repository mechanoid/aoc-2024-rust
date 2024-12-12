use core::panic;

mod util;
pub use util::parse;

#[derive(Debug, PartialEq)]
pub struct Equation {
    result: i64,
    operands: Vec<i64>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
}

fn operator_by_index(i: i8) -> Operator {
    return match i {
        0 => Operator::Add,
        1 => Operator::Multiply,
        _ => panic!("unknown operator"),
    };
}

fn increment(permutation: &Vec<i8>, base: i8, position: usize) -> Option<Vec<i8>> {
    let mut permutation = permutation.clone();

    let update = permutation[position] + 1;

    if update <= (base - 1) {
        permutation[position] = update;
        return Some(permutation);
    } else if position > 0 {
        // handle overflow
        let next_position = position - 1;

        // reset lower indexes
        for (i, _) in permutation[position..].to_vec().iter().enumerate() {
            permutation[i + position] = 0;
        }

        return increment(&permutation, base, next_position);
    }

    return None;
}

fn compute_result_for_permutation(operands: &Vec<i64>, permutation: &Vec<i8>) -> i64 {
    let mut result = 0;

    for i in 0..operands.len() - 1 {
        let a = operands[i];
        let b = operands[i + 1];

        if i == 0 {
            result = a;
        }
        let operator = operator_by_index(permutation[i]);

        result = match operator {
            Operator::Add => result + b,
            Operator::Multiply => result * b,
        }
    }

    return result;
}

fn try_configuration(
    Equation { operands, result }: &Equation,
    operators: &Vec<Operator>,
) -> Result<i64, String> {
    let mut operator_permutation: Vec<i8> = vec![0; operands.len()];

    while let Some(perm) = increment(
        &operator_permutation,
        operators.len() as i8,
        operands.len() - 1,
    ) {
        operator_permutation = perm;

        let config_result = compute_result_for_permutation(&operands, &operator_permutation);

        if *result == config_result {
            return Ok(config_result);
        }
    }

    return Err("failed to compute any results".to_string());
}

pub fn calibrate(equations: Vec<Equation>) -> i64 {
    let mut result = 0;
    let operators = [Operator::Add, Operator::Multiply].to_vec();

    for equation in equations {
        if let Ok(intermediante_result) = try_configuration(&equation, &operators) {
            result += intermediante_result;
        }
    }
    return result;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use crate::calibration::{calibrate, try_configuration, util::parse, Equation, Operator};

    #[test]
    fn test_try_configuration() {
        let equation = Equation {
            result: 190,
            operands: vec![10, 19],
        };
        let operators = [Operator::Add, Operator::Multiply].to_vec();
        let result = try_configuration(&equation, &operators).unwrap();
        assert_eq!(result, 190);

        let equation = Equation {
            result: 3267,
            operands: vec![81, 40, 27],
        };
        let operators = [Operator::Add, Operator::Multiply].to_vec();
        let result = try_configuration(&equation, &operators).unwrap();
        assert_eq!(result, 3267);
    }

    #[test]
    fn test_calibrate() {
        let equations = parse(
            "190: 10 19
                        3267: 81 40 27
                        83: 17 5
                        156: 15 6
                        7290: 6 8 6 15
                        161011: 16 10 13
                        192: 17 8 14
                        21037: 9 7 18 13
                        292: 11 6 16 20",
        );

        let result = calibrate(equations);
        assert_eq!(result, 3749)
    }
}
