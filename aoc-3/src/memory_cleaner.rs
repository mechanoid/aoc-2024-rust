use regex::Regex;

pub fn evaluate_line(line: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut results = 0;

    for (_, [m1, m2]) in pattern.captures_iter(line).map(|c| c.extract()) {
        let m1 = m1.parse::<i32>().unwrap();
        let m2 = m2.parse::<i32>().unwrap();
        results += m1 * m2;
    }

    return results;
}

pub fn evaluate_memory(memory_dump: &str) -> i32 {
    let memory_dump: Vec<&str> = memory_dump.trim().split("\n").collect();
    let mut result = 0;

    for line in memory_dump {
        result += evaluate_line(line);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::evaluate_memory;

    #[test]
    fn test_evaluate_memory() {
        let memory_dump_example =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = evaluate_memory(&memory_dump_example);

        assert_eq!(result, 161);
    }
}
