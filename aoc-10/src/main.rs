use std::fs;

mod routing;
use routing::{parse, trail_head_scoring};

fn main() {
    let topographic_map = fs::read_to_string("./data/topographic_map.txt")
        .expect("Should have been able to read the file");
    let topographic_map = parse(&topographic_map);
    let scoring = trail_head_scoring(&topographic_map);
    println!("trail head scoring for map: {} ", scoring);
}
