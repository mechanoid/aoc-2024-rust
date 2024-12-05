use std::fs;
mod safety_protocoll_printing;
use safety_protocoll_printing::summarize_valid_updates;

fn main() {
    let update_plan = fs::read_to_string("./data/updates_plan.txt")
        .expect("Should have been able to read the file");

    let result = summarize_valid_updates(&update_plan);
    println!(
        "sum of middlepage numbers for correctly ordered pages is: {}",
        result
    );
}
