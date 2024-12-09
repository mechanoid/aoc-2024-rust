use std::fs;
mod guard_avoidance_1;
use guard_avoidance_1::{parse_map, predict_path_positions, Map};

mod guard_avoidance_2;
use guard_avoidance_2::find_loops;

fn main() {
    let original_map =
        fs::read_to_string("./data/map.txt").expect("Should have been able to read the file");
    let map: Map = parse_map(&original_map);

    let (step_count, steps) = predict_path_positions(&map);
    println!("guard will visit {} positions on its way", step_count);

    let loops = find_loops(&map, steps);
    println!("found {} possible loops", loops)
}
