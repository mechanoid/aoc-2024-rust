use regex::Regex;

fn count_matches(text: &str, pattern: &str) -> u32 {
    let pattern = Regex::new(pattern).unwrap();

    let matches: Vec<&str> = pattern.find_iter(text).map(|m| m.as_str()).collect();
    return matches.len() as u32;
}

fn flip_text_by_90(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }

    let input: Vec<&str> = input.split("\n").collect();

    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut rotated: Vec<String> = Vec::with_capacity(num_cols);

    for col in 0..num_cols {
        let mut new_row = String::with_capacity(num_rows);
        for row in 0..num_rows {
            new_row.push(input[row].chars().nth(col).unwrap());
        }

        rotated.push(new_row);
    }

    return rotated.join("\n").to_string();
}

fn flip_text_by_45(input: &str) -> String {
    let input: Vec<&str> = input.split("\n").collect();
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut output: Vec<String> = vec![String::new(); num_rows + num_cols - 1];

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, ch) in row.chars().rev().enumerate() {
            let diagonal_idx = row_idx + col_idx;
            output[diagonal_idx].push(ch);
        }
    }

    let output = output.join("\n").to_string();
    return output;
}

fn flip_text_by_minus_45(input: &str) -> String {
    let input: Vec<&str> = input.split("\n").collect();
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut output: Vec<String> = vec![String::new(); num_rows + num_cols - 1];

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            let diagonal_idx = row_idx + col_idx;
            output[diagonal_idx].push(ch);
        }
    }

    let output = output.join("\n").to_string();
    return output;
}

fn search_horizontally(text: &str) -> u32 {
    let mut sum = 0;

    sum += count_matches(&text, r"XMAS");
    sum += count_matches(&text, r"SAMX");

    return sum;
}

fn search_vertically(text: &str) -> u32 {
    let text_turned_by_90 = flip_text_by_90(text);
    return search_horizontally(&text_turned_by_90);
}

fn search_diagonal_top_left_to_bottom_right(text: &str) -> u32 {
    let text_turned_by_45 = flip_text_by_45(&text);
    return search_horizontally(&text_turned_by_45);
}

fn search_diagonal_bottom_left_to_top_right(text: &str) -> u32 {
    let text_turned_by_minus_45 = flip_text_by_minus_45(&text);
    return search_horizontally(&text_turned_by_minus_45);
}

pub fn xmas_search(text: &str) -> u32 {
    let text = text.trim();
    let mut sum = 0;

    sum += search_horizontally(&text);
    sum += search_vertically(&text);

    sum += search_diagonal_top_left_to_bottom_right(text);
    sum += search_diagonal_bottom_left_to_top_right(&text);

    return sum;
}

// ############################################################################################
// ############################################################################################
// # TESTS                                                                                    #
// ############################################################################################
// ############################################################################################

#[cfg(test)]
mod tests {
    use crate::xmas_search::{
        flip_text_by_45, flip_text_by_90, flip_text_by_minus_45, search_horizontally, xmas_search,
    };

    #[test]
    fn test_flip_text_by_45() {
        let text = "XAAA
BMBB
CCAC
DDDS";
        let text = flip_text_by_45(&text);
        assert_eq!(
            text,
            "A
AB
ABC
XMAS
BCD
CD
D"
        );
    }

    #[test]
    fn test_flip_text_by_360() {
        let text = "AAAA
BBBB
CCCC
DDDD";
        let text = flip_text_by_90(&text);
        let text = flip_text_by_90(&text);
        let text = flip_text_by_90(&text);
        let text = flip_text_by_90(&text);
        assert_eq!(
            text,
            "AAAA
BBBB
CCCC
DDDD"
        );
    }

    #[test]
    fn test_flip_text_by_minus_45() {
        let text = "AAAX
BBMB
CACC
SDDD";
        let flipped = flip_text_by_minus_45(text);
        assert_eq!(
            flipped,
            "A
AB
ABC
XMAS
BCD
CD
D"
        );
    }

    #[test]
    fn test_flip_text_by_90() {
        let text = "AAAA
BBBB
CCCC
DDDD";
        let flipped = flip_text_by_90(text);
        assert_eq!(
            flipped,
            "ABCD
ABCD
ABCD
ABCD"
        );
    }

    #[test]
    fn test_search_horizontally() {
        let text = "....XXMAS.
.SAMX.....
XMASAMX...
.....XMASX";
        let xmas_count = search_horizontally(text);
        assert_eq!(xmas_count, 5);
    }

    #[test]
    fn test_xmas_search() {
        let text = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let xmas_count = xmas_search(text);
        assert_eq!(xmas_count, 18);
    }
}
