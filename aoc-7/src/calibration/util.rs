use super::Equation;

pub fn parse(equations: &str) -> Vec<Equation> {
    return equations
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let line = line.trim().split(":").collect::<Vec<&str>>();
            let result = line.first().unwrap();
            let result = result.parse::<i64>().unwrap();

            let operands = line.last().unwrap();
            let operands = operands
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|op| op.parse::<i64>().unwrap())
                .collect();

            return Equation {
                result: result,
                operands: operands,
            };
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{parse, Equation};

    #[test]
    fn test_parsing() {
        let equations = " 190: 10 19
                                3267: 81 40 27";
        let result = parse(&equations);

        assert_eq!(
            result,
            vec![
                Equation {
                    result: 190,
                    operands: vec![10, 19]
                },
                Equation {
                    result: 3267,
                    operands: vec![81, 40, 27]
                }
            ]
        )
    }
}
