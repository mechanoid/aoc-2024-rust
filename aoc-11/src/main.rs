mod warping;
use std::fs;
use warping::{parse, stone_count_for_n_blinks};

// #[tokio::main]
fn main() {
    let blink_stones = fs::read_to_string("./data/warp_stones.txt")
        .expect("Should have been able to read the file");
    let blink_stones = parse(&blink_stones);

    // let blinked = blink_n_times(&blink_stones, 25).await;
    // println!(
    //     "There are {} stones after 25 times blinking!",
    //     blinked.len()
    // );

    // let blinked = stone_count_for_n_blinks(&blink_stones, 25);
    // println!("There are {} stones after 25 times blinking!", blinked);

    let blinked = stone_count_for_n_blinks(&blink_stones, 75);
    println!("There are {} stones after 75 times blinking!", blinked);
}
