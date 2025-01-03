mod fencing;
use fencing::{fencing_price, parse};
use std::fs;

// #[tokio::main]
fn main() {
    let map =
        fs::read_to_string("./data/farm_map.txt").expect("Should have been able to read the file");
    let map = parse(&map);
    let price = fencing_price(&map);
    println!("all fences together cost {} elf-dollars", price);
}
