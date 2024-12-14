use std::fs;

mod frequencies;
use frequencies::{find_antinodes, parse};

fn main() {
    let antenna_map = fs::read_to_string("./data/antenna_map.txt")
        .expect("Should have been able to read the file");

    let antenna_map = parse(&antenna_map);

    let anti_node_count = find_antinodes(antenna_map).unwrap();

    println!("found {} distinct antinodes", anti_node_count);
}
