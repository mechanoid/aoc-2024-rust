use std::fs;

mod calibration;
use calibration::{calibrate, parse};

fn main() {
    let equations =
        fs::read_to_string("./data/equations.txt").expect("Should have been able to read the file");
    let equations = parse(&equations);
    let result = calibrate(equations);
    println!("result: {}", result)
}
