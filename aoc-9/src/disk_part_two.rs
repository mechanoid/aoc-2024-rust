#[derive(Clone, Copy, Debug)]
struct Block {
    id: Option<u64>,
    length: u64,
    is_file: bool,
    freed: bool,
    copied: bool,
}

fn new_free_block(length: u64) -> Block {
    Block {
        id: None,
        length,
        is_file: false,
        freed: false,
        copied: false,
    }
}

fn new_file(id: u64, length: u64) -> Block {
    Block {
        id: Some(id),
        length,
        is_file: true,
        freed: false,
        copied: false,
    }
}

fn even(num: u8) -> bool {
    num % 2 == 0
}

fn parse_disk_map(disk_map: &str) -> Vec<Block> {
    let mut id: u64 = 0;

    let mut parsed: Vec<Block> = vec![];

    for (index, number) in disk_map.trim().chars().into_iter().enumerate() {
        let length = number.to_string().parse::<u64>().unwrap();

        if even(index as u8) {
            parsed.push(new_file(id, length));
            id += 1;
        } else {
            parsed.push(new_free_block(length));
        }
    }

    return parsed;
}

fn files_in_reverse_order(disk_map: &Vec<Block>) -> Vec<Block> {
    disk_map
        .clone()
        .iter()
        .filter(|block| block.is_file)
        .rev()
        .map(|b| *b)
        .collect()
}

fn find_free_block(blocks: &Vec<Block>, min_length: u64) -> Option<(usize, Block)> {
    if let Some((idx, free_slot)) = blocks
        .iter()
        .enumerate()
        .find(|(_, block)| !block.is_file && block.length >= min_length)
    {
        return Some((idx, *free_slot));
    }

    return None;
}

fn defragment(disk_map: Vec<Block>) -> Vec<Block> {
    let mut defragmented = disk_map.clone();

    for candidate in files_in_reverse_order(&disk_map) {
        if let Some((slot_index, free_slot)) = find_free_block(&defragmented, candidate.length) {
            // find current index and get mutable reference of file (after each mutation)
            if let Some((idx, file)) = defragmented
                .iter_mut()
                .enumerate()
                .find(|(_, f)| f.id == candidate.id)
            {
                if slot_index >= idx {
                    continue;
                }

                let mut original_file = file.clone();
                original_file.copied = true;

                file.id = None;
                file.freed = true;

                // add a free block after replacing the free slot with the file in the length of the remaining part.
                if free_slot.length > file.length {
                    let length = free_slot.length - file.length;
                    defragmented.insert(slot_index + 1, new_free_block(length));
                }

                defragmented[slot_index] = original_file;
            }
        }
    }

    return defragmented;
}

fn checksum(disk_map: Vec<Block>) -> u64 {
    let mut sum = 0;
    let mut index = 0;

    for block in disk_map.iter() {
        let id = block.id.unwrap_or(0);

        for _ in 0..block.length {
            sum += index * id;
            index += 1;
        }
    }
    return sum;
}

pub fn defragment_by_file(disk_map: &str) -> u64 {
    let disk_map = parse_disk_map(disk_map);
    let disk_map = defragment(disk_map);
    // println!("checked:\n\n{}", render_blocks(&disk_map));
    return checksum(disk_map);
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {

    use super::{checksum, defragment, defragment_by_file, parse_disk_map, Block};

    // for ids > 9 the rendered ids take more place, this is basically wrong
    // and we use that only to be able to write down the expectation as string
    fn render_blocks(blocks: &Vec<Block>) -> String {
        return blocks
            .iter()
            .map(|block| {
                if let Some(id) = block.id {
                    return vec![id.to_string(); block.length as usize].join("");
                } else {
                    return vec!["."; block.length as usize].join("");
                }
            })
            .collect::<Vec<String>>()
            .join("");
    }

    #[test]
    fn test_parsing() {
        let compact_format = "2333133121414131402";
        let result = parse_disk_map(compact_format);
        assert_eq!(
            render_blocks(&result),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_compacting() {
        let compact_format = "2333133121414131402";
        let disk_layout = parse_disk_map(compact_format);
        let compacted = defragment(disk_layout);
        assert_eq!(
            render_blocks(&compacted),
            "00992111777.44.333....5555.6666.....8888.."
        );

        let compact_format = "11111";
        let disk_layout = parse_disk_map(compact_format);
        let compacted = defragment(disk_layout);
        assert_eq!(render_blocks(&compacted), "021..");

        let compact_format = "111111111111111111111111111";
        let disk_layout = parse_disk_map(compact_format);
        let compacted = defragment(disk_layout);
        assert_eq!(render_blocks(&compacted), "013112211310495867.............");
    }

    #[test]
    fn test_checksum() {
        let compact_format = "11111";
        let disk_layout = parse_disk_map(compact_format);
        let compacted = defragment(disk_layout);
        let result = checksum(compacted);
        assert_eq!(result, 4);

        let compact_format = "999";
        let disk_layout = parse_disk_map(compact_format);
        let compacted = defragment(disk_layout);

        let result = checksum(compacted);
        assert_eq!(result, 117);
    }

    #[test]
    fn test_compact_by_file() {
        let compact_format = "2333133121414131402";
        let result = defragment_by_file(compact_format);
        assert_eq!(result, 2858);

        let compact_format = "233313312141413140219";
        let result = defragment_by_file(compact_format);
        assert_eq!(result, 7088);
    }
}
