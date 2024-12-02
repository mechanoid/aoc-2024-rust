enum Mode {
    Increasing,
    Decreasing,
    Failing,
}
fn check_levels(report: &str) -> Mode {
    let levels: Vec<&str> = report.split(" ").collect();
    let levels = levels
        .into_iter()
        .map(|level| level.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();

    let mut current = levels[0];
    let levels = &levels[1..];

    let mut mode = match levels[1] {
        l if l > current => Mode::Increasing,
        l if l < current => Mode::Decreasing,
        _ => Mode::Failing,
    };

    for level in levels {
        let level = *level;

        match mode {
            Mode::Increasing if level > current && level <= (current + 3) => current = level,
            Mode::Decreasing if level < current && level >= (current - 3) => current = level,
            _ => {
                mode = Mode::Failing;
            }
        }
    }

    return mode;
}

pub fn safety_report(reports: &str) -> usize {
    let reports: Vec<&str> = reports.trim().split("\n").collect();
    let mut safe_reports = 0;

    for report in reports {
        let report_mode = check_levels(&report);
        safe_reports += match report_mode {
            Mode::Decreasing => 1,
            Mode::Increasing => 1,
            Mode::Failing => 0,
        }
    }

    return safe_reports;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_levels() {
        let report = "7 6 4 2 1";
        let result = super::check_levels(&report);
        assert!(matches!(result, super::Mode::Decreasing));

        let report = "1 2 7 8 9";
        let result = super::check_levels(&report);
        assert!(matches!(result, super::Mode::Failing));

        let report = "9 7 6 2 1";
        let result = super::check_levels(&report);
        assert!(matches!(result, super::Mode::Failing));

        let report = "1 3 2 4 5";
        let result = super::check_levels(&report);
        assert!(matches!(result, super::Mode::Failing));

        let report = "8 6 4 4 1";
        let result = super::check_levels(&report);
        assert!(matches!(result, super::Mode::Failing));

        let report = "1 3 6 7 9";
        let result = super::check_levels(&report);
        assert!(matches!(result, super::Mode::Increasing));
    }

    #[test]
    fn test_safety_report() {
        let reports = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = super::safety_report(&reports);
        assert_eq!(result, 2);
    }
}
