fn parse_instruction(instruction: &str) -> (u32, u32) {
    let instruction: Vec<&str> = instruction.trim().split("|").collect();
    let instruction: Vec<u32> = instruction
        .iter()
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    if instruction.len() > 2 {
        panic!("order instruction contains more then two pages")
    };

    return (instruction[0], instruction[1]);
}

fn parse_sorting_order(order_instructions: &str) -> Vec<(u32, u32)> {
    let instructions: Vec<&str> = order_instructions.split("\n").collect();

    return instructions
        .iter()
        .map(|i| parse_instruction(i))
        .collect::<Vec<(u32, u32)>>();
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    let updates: Vec<&str> = updates.split("\n").collect();
    let mut result: Vec<Vec<u32>> = vec![];
    for update in updates {
        let parsed_update: Vec<u32> = update
            .trim()
            .split(",")
            .into_iter()
            .map(|u| u.parse::<u32>().unwrap())
            .collect();

        result.push(parsed_update);
    }
    return result;
}

fn check_update_for_order(
    update: &Vec<u32>,
    order_instructions: &Vec<(u32, u32)>,
) -> Option<Vec<u32>> {
    let mut res = true;
    if update.iter().all(|current| {
        for (before, after) in order_instructions {
            if !(current == before || current == after) {
                // current instruction does not apply on that element
                continue;
            } else if !(update.contains(before) && update.contains(after)) {
                // current instruction has one element, not even in this update, so it does not apply to it.
                continue;
            }

            // because of the above implemented checks, we can safely unwrap the indexes here
            let current_index = update.iter().position(|u| u == current).unwrap();
            let before_index = update.iter().position(|u| u == before).unwrap();
            let after_index = update.iter().position(|u| u == after).unwrap();

            if current == before && current_index > after_index {
                res = false
            } else if current == after && current_index < before_index {
                res = false
            }
        }
        return res;
    }) {
        return Some(update.clone());
    }
    return None;
}

// returns only valid updates
fn get_valid_updates(
    updates: &Vec<Vec<u32>>,
    order_instructions: &Vec<(u32, u32)>,
) -> Vec<Vec<u32>> {
    let mut results: Vec<Vec<u32>> = vec![];

    for update in updates {
        if let Some(update) = check_update_for_order(&update, &order_instructions) {
            results.push(update);
        }
    }

    return results;
}

fn middle_page(update: &Vec<u32>) -> u32 {
    return update[update.len() / 2]; // zero based
}

fn summarize_updates(updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for update in updates {
        sum += middle_page(&update);
    }

    return sum;
}

pub fn summarize_valid_updates(update_plan: &str) -> u32 {
    let updates = update_plan.trim();

    if let [order_instructions, updates] = updates.split("\n\n").collect::<Vec<&str>>().as_slice() {
        let updates = parse_updates(&updates);
        let order_instructions = parse_sorting_order(&order_instructions);
        let valid_updates = get_valid_updates(&updates, &order_instructions);
        return summarize_updates(&valid_updates);
    };

    return 0;
}

#[cfg(test)]
mod tests {
    use super::{
        check_update_for_order, get_valid_updates, parse_sorting_order, summarize_updates,
        summarize_valid_updates,
    };

    #[test]
    fn test_summarize_updates() {
        let example_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];

        let result = summarize_updates(&example_updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_check_update_for_order() {
        let example_order = vec![(47, 53), (97, 13), (97, 61), (97, 47), (75, 29)];
        let example_update: Vec<u32> = vec![75, 47, 61, 53, 29];

        let result = check_update_for_order(&example_update, &example_order);
        assert_eq!(result.unwrap(), vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_get_valid_updates() {
        let example_order = vec![(47, 53), (97, 13), (97, 61), (97, 47), (75, 29)];
        let example_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13, 97],
        ];

        let result = get_valid_updates(&example_updates, &example_order);
        assert_eq!(
            result,
            vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]]
        );
    }

    #[test]
    fn test_parse_sorting_order() {
        let example_sorting_order = "47|53
97|13
97|61
97|47
75|29";
        let result = parse_sorting_order(&example_sorting_order);
        assert_eq!(
            result,
            vec![(47, 53), (97, 13), (97, 61), (97, 47), (75, 29)]
        );
    }

    #[test]
    fn test_summarize_valid_updates() {
        let example_plan = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = summarize_valid_updates(&example_plan);

        assert_eq!(result, 143);
    }
}
