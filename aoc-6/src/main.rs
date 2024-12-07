use std::fs;
mod guard_avoidance;
use guard_avoidance::predict_path_positions;

fn main() {
    let mut map =
        fs::read_to_string("./data/map.txt").expect("Should have been able to read the file");

    let (_, _, steps) = predict_path_positions(&mut map);

    println!("guard will visit {} positions on its war", steps);
}
