use std::fs;

mod data_analysis;

fn main() {
    let reports =
        fs::read_to_string("./data/reports.txt").expect("Should have been able to read the file");

    let (safe_reports, overall_count) = data_analysis::safety_report(&reports, &false);
    let (safe_reports_dampened, _) = data_analysis::safety_report(&reports, &true);

    println!(
        "Safe reports (undampened): {} from {}",
        safe_reports, overall_count
    );

    println!(
        "Safe reports (dampened): {} from {}",
        safe_reports_dampened, overall_count
    );
}
