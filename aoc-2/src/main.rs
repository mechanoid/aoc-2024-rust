use std::fs;

mod data_analysis;

fn main() {
    let reports =
        fs::read_to_string("./data/reports.txt").expect("Should have been able to read the file");

    let safe_reports = data_analysis::safety_report(&reports);
    println!("Safe reports: {}", safe_reports);
}
