use std::fs;

mod xmas_search;
use xmas_search::x_mas_search;
use xmas_search::xmas_search;

fn main() {
    let words =
        fs::read_to_string("./data/words.txt").expect("Should have been able to read the file");

    let word_search_result = xmas_search(&words);
    println!("XMAS/SAMX found: {} times", word_search_result);

    let word_search_result = x_mas_search(&words);
    println!("MAS as X found: {} times", word_search_result);
}
