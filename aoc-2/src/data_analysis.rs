#[derive(Debug)]
enum Mode {
    Increasing,
    Decreasing,
    Failing,
    Undecided,
}

fn replace_level_at_index(levels: &Vec<i8>, index: usize) -> Vec<i8> {
    let mut levels = levels.to_vec();
    levels.remove(index);
    return levels;
}

fn check_levels_with_dampener(levels: Vec<i8>, dampened: &bool, dampener_used: &bool) -> Mode {
    let mut current = levels[0];
    let levels_without_leading = &levels[1..];

    let mode = match levels_without_leading[0] {
        l if l > current => Mode::Increasing,
        l if l < current => Mode::Decreasing,
        _ => Mode::Undecided,
    };

    for level in levels_without_leading {
        let level = *level;

        match mode {
            Mode::Increasing if level > current && level <= (current + 3) => current = level,
            Mode::Decreasing if level < current && level >= (current - 3) => current = level,
            _ => {
                if *dampened && !*dampener_used {
                    for i in 0..levels.len() {
                        let levels_without_number_at_index = replace_level_at_index(&levels, i);

                        let mode_without_number = check_levels_with_dampener(
                            levels_without_number_at_index.to_vec(),
                            &true,
                            &true,
                        );

                        match mode_without_number {
                            Mode::Decreasing => return mode_without_number,
                            Mode::Increasing => return mode_without_number,
                            _ => (),
                        };
                    }
                }

                return Mode::Failing;
            }
        }
    }

    return match mode {
        Mode::Undecided => Mode::Failing,
        _ => mode,
    };
}

fn check_levels(report: &str, dampened: &bool) -> Mode {
    let levels: Vec<&str> = report.split(" ").collect();
    let levels = levels
        .into_iter()
        .map(|level| level.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();

    return check_levels_with_dampener(levels, dampened, &false);
}

pub fn safety_report(reports: &str, dampened: &bool) -> (usize, usize) {
    let reports: Vec<&str> = reports.trim().split("\n").collect();
    let mut safe_reports = 0;

    for report in &reports {
        let report_mode = check_levels(&report, &dampened);
        safe_reports += match report_mode {
            Mode::Decreasing => 1,
            Mode::Increasing => 1,
            Mode::Failing => 0,
            Mode::Undecided => {
                panic!("undecided mode has not been resolved")
            }
        };
    }

    return (safe_reports, reports.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_levels_dampened() {
        let report = "8 8 10 11 14 17";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Increasing),
            "errorneous mode: {:?}",
            result
        );

        let report = "10 7 4 3 2 1";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Decreasing),
            "errorneous mode: {:?}",
            result
        );

        let report = "7 6 4 2 1";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Decreasing),
            "errorneous mode: {:?}",
            result
        );

        let report = "1 2 7 8 9";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Failing),
            "errorneous mode: {:?}",
            result
        );

        let report = "9 7 6 2 1";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Failing),
            "errorneous mode: {:?}",
            result
        );

        let report = "1 3 2 4 5";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Increasing),
            "errorneous mode: {:?}",
            result
        );

        let report = "1 3 4 7 10 10";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Increasing),
            "errorneous mode: {:?}",
            result
        );

        let report = "8 6 4 4 1";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Decreasing),
            "errorneous mode: {:?}",
            result
        );

        let report = "1 3 6 7 9";
        let result = super::check_levels(&report, &true);
        assert!(
            matches!(result, super::Mode::Increasing),
            "errorneous mode: {:?}",
            result
        );
    }

    #[test]
    fn test_check_levels_undampened() {
        let report = "7 6 4 2 1";
        let result = super::check_levels(&report, &false);
        assert!(matches!(result, super::Mode::Decreasing));

        let report = "1 2 7 8 9";
        let result = super::check_levels(&report, &false);
        assert!(matches!(result, super::Mode::Failing));

        let report = "9 7 6 2 1";
        let result = super::check_levels(&report, &false);
        assert!(matches!(result, super::Mode::Failing));

        let report = "1 3 2 4 5";
        let result = super::check_levels(&report, &false);
        assert!(matches!(result, super::Mode::Failing));

        let report = "8 6 4 4 1";
        let result = super::check_levels(&report, &false);
        assert!(matches!(result, super::Mode::Failing));

        let report = "1 3 6 7 9";
        let result = super::check_levels(&report, &false);
        assert!(matches!(result, super::Mode::Increasing));
    }

    #[test]
    fn test_safety_report_dampened() {
        let reports = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = super::safety_report(&reports, &true);
        assert_eq!(result, (4, 6));
    }

    #[test]
    fn test_safety_report_undampened() {
        let reports = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = super::safety_report(&reports, &false);
        assert_eq!(result, (2, 6));
    }
}
