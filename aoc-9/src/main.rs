use std::fs;

use disk::compact_and_update_checksum;

mod disk;

fn main() {
    let disk_map =
        fs::read_to_string("./data/disk_map.txt").expect("Should have been able to read the file");

    let result = compact_and_update_checksum(disk_map.as_str());

    println!("checksum: {}", result);
}
