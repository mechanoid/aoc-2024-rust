use std::fs;
mod list_comparison;

fn main() {
    let locations = fs::read_to_string("./data/locations_list.txt")
        .expect("Should have been able to read the file");

    let td = list_comparison::total_distance(&locations);
    println!("total distance: {}", td)
}
