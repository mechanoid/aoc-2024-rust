mod warping;
use std::fs;
use warping::{blink_n_times, parse};

fn main() {
    let blink_stones = fs::read_to_string("./data/warp_stones.txt")
        .expect("Should have been able to read the file");
    let blink_stones = parse(&blink_stones);

    let blinked = blink_n_times(&blink_stones, 25);
    println!(
        "There are {} stones after 25 times blinking!",
        blinked.len()
    );

    let blinked = blink_n_times(&blink_stones, 75);
    println!(
        "There are {} stones after 75 times blinking!",
        blinked.len()
    );
}
