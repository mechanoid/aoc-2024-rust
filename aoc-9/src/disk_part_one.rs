fn parse_disk_map(disk_map: &str) -> Vec<Option<u64>> {
    let mut id: u64 = 0;

    let mut parsed: Vec<Option<u64>> = vec![];

    for (index, number) in disk_map.trim().chars().into_iter().enumerate() {
        let size = number.to_string().parse::<usize>().unwrap();
        let rest = index % 2;

        let mut block = None;

        if rest == 0 {
            block = Some(id);
            id += 1;
        }

        vec![1; size].iter().for_each(|_| parsed.push(block));
    }

    return parsed;
}

fn compact(disk_map: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut compacted = disk_map.clone();

    for (rev_index, number_from_behind) in disk_map.iter().rev().enumerate() {
        if let Some(number_from_behind) = number_from_behind {
            let next_free_space = compacted.iter().enumerate().find(|(_, el)| el.is_none());

            if let Some((next_dot_index, _)) = next_free_space {
                let original_index = disk_map.len() - rev_index - 1;

                if original_index < next_dot_index {
                    break; // replaced all possibilities. Rest of dots is in the end of the string
                }

                compacted[next_dot_index] = Some(*number_from_behind);
                compacted[original_index] = None;
            }
        } else {
            continue;
        }
    }

    return compacted;
}

pub fn compact_blockwise(disk_map: &str) -> u64 {
    let disk_map = parse_disk_map(disk_map);
    let disk_map = compact(disk_map);

    let mut sum = 0;

    for (i, n) in disk_map.iter().enumerate() {
        if let Some(n) = n {
            sum += i as u64 * *n;
        }
    }

    return sum;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use super::{compact, compact_blockwise, parse_disk_map};

    fn render_number_options(numbers: Vec<Option<u64>>) -> String {
        return numbers
            .iter()
            .map(|n| {
                if let Some(number) = n {
                    return number.to_string();
                } else {
                    return ".".to_string();
                }
            })
            .collect::<Vec<String>>()
            .join("");
    }

    #[test]
    fn test_parse_disk_map() {
        let compact_format = "2333133121414131402";
        let result = parse_disk_map(compact_format);
        assert_eq!(
            render_number_options(result),
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        );
    }

    #[test]
    fn test_compaction() {
        let compact_format = "12345";
        let disk_layout = parse_disk_map(compact_format);
        let result = compact(disk_layout);

        assert_eq!(render_number_options(result), "022111222......".to_string());

        let compact_format = "2333133121414131402";
        let disk_layout = parse_disk_map(compact_format);
        let result = compact(disk_layout);

        assert_eq!(
            render_number_options(result),
            "0099811188827773336446555566..............".to_string()
        );
    }

    #[test]
    fn test_compact_blockwise() {
        let compact_format = "2333133121414131402";
        let result = compact_blockwise(compact_format);
        assert_eq!(result, 1928);
    }
}
