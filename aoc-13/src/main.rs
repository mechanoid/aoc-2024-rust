mod claw_machines;

use claw_machines::{min_tokens_for_max_prices, parse};
use std::fs;

fn main() {
    let machines = fs::read_to_string("./data/claw_machines.txt")
        .expect("Should have been able to read the file");
    let machines = parse(&machines);

    let tokens = min_tokens_for_max_prices(&machines);
    println!("you'll need {} tokens", tokens);
}
