use std::fs;
mod guard_avoidance_1;
use guard_avoidance_1::{parse_map, predict_path_positions, show_map, Map};

mod guard_avoidance_2;
use guard_avoidance_2::find_loops;

fn main() {
    let original_map =
        fs::read_to_string("./data/map.txt").expect("Should have been able to read the file");
    let mut map: Map = parse_map(&original_map);

    let (original_guards_path, _, steps) = predict_path_positions(&mut map);
    println!("guard will visit {} positions on its way", steps);
    println!(
        "Original Guards Path:\n{}\n",
        show_map(&original_guards_path)
    );

    let loops = find_loops(&map, &original_guards_path);

    println!("found {} possible loops", loops)
}
