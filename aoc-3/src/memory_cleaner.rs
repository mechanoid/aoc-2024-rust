use regex::Regex;

fn evaluate_multiplies(line: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = 0;

    for (_, [m1, m2]) in pattern.captures_iter(line).map(|c| c.extract()) {
        let m1 = m1.parse::<i32>().unwrap();
        let m2 = m2.parse::<i32>().unwrap();
        results += m1 * m2;
    }

    return results;
}

pub fn evaluate_memory_with_conditionals(line: &str) -> i32 {
    let mut results = 0;

    let line_splitted_by_donts: Vec<&str> = line.split("don't()").collect();
    // always count muls until first don't()
    results += evaluate_multiplies(&line_splitted_by_donts[0]);

    if line_splitted_by_donts.len() > 1 {
        // go over remaining parts after each don't()
        for index in 1..line_splitted_by_donts.len() {
            let part_after_a_dont = line_splitted_by_donts[index];

            // if it contains a do(), let's count those muls after the do
            if let Some(begin_of_first_do_after_dont) = part_after_a_dont.find("do()") {
                let reenabled_line = &part_after_a_dont[(begin_of_first_do_after_dont + 4)..];

                results += evaluate_multiplies(&reenabled_line);
            }
        }
    }

    return results;
}

pub fn evaluate_memory(memory_dump: &str) -> i32 {
    return evaluate_multiplies(memory_dump);
}

#[cfg(test)]
mod tests {
    use super::evaluate_memory;
    use super::evaluate_memory_with_conditionals;

    #[test]
    fn test_evaluate_memory_with_conditionals() {
        let memory_dump_example =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = evaluate_memory_with_conditionals(&memory_dump_example);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_evaluate_memory() {
        let memory_dump_example =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = evaluate_memory(&memory_dump_example);
        assert_eq!(result, 161);

        let memory_dump_example =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = evaluate_memory(&memory_dump_example);
        assert_eq!(result, 322);

        let memory_dump_example =
            ":^from()??from()from()^mul(316,793)>'/when()+mul(199,368)[~mul(539,838) ^(*??%mul(162,286){mul(647,357)who()$(>what()?,why()>from()mul(239,449){select()who()mul(408,790)mul(567,879)";
        let result = evaluate_memory(&memory_dump_example);
        assert_eq!(result, 1980837);
    }
}
