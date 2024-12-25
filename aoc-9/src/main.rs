use std::fs;

// use disk_part_one::compact_blockwise;
use disk_part_two::defragment_by_file;

mod disk_part_one;
mod disk_part_two;

fn main() {
    let disk_map =
        fs::read_to_string("./data/disk_map.txt").expect("Should have been able to read the file");

    // let result = compact_blockwise(disk_map.as_str());
    // println!("checksum: {}", result);

    let result = defragment_by_file(disk_map.as_str());
    println!("checksum: {}", result);
}
