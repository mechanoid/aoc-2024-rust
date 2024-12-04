use std::fs;

mod memory_cleaner;
use memory_cleaner::{evaluate_memory, evaluate_memory_with_conditionals};

fn main() {
    let memory_dump = fs::read_to_string("./data/memory_dump.txt")
        .expect("Should have been able to read the file");

    let result = evaluate_memory(&memory_dump);
    println!("result after memory cleanup: {}", result);

    let result = evaluate_memory_with_conditionals(&memory_dump);
    println!("result after memory cleanup with conditionals: {}", result);
}
