use regex::Regex;

// ############################################################################################
// ############################################################################################
// # VARIANT 1                                                                                #
// ############################################################################################
// ############################################################################################

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

fn search_horizontally(text: &str, patterns: &(&str, &str)) -> u32 {
    let mut sum = 0;

    sum += count_matches(&text, patterns.0);
    sum += count_matches(&text, patterns.1);

    return sum;
}

fn search_vertically(text: &str, patterns: &(&str, &str)) -> u32 {
    let text_turned_by_90 = flip_text_by_90(text);
    return search_horizontally(&text_turned_by_90, &patterns);
}

fn search_diagonal_top_left_to_bottom_right(text: &str, patterns: &(&str, &str)) -> u32 {
    let text_turned_by_45 = flip_text_by_45(&text);
    return search_horizontally(&text_turned_by_45, &patterns);
}

fn search_diagonal_bottom_left_to_top_right(text: &str, patterns: &(&str, &str)) -> u32 {
    let text_turned_by_minus_45 = flip_text_by_minus_45(&text);
    return search_horizontally(&text_turned_by_minus_45, &patterns);
}

pub fn xmas_search(text: &str) -> u32 {
    let text = text.trim();
    let mut sum = 0;
    let patterns = (r"XMAS", r"SAMX");
    sum += search_horizontally(&text, &patterns);
    sum += search_vertically(&text, &patterns);

    sum += search_diagonal_top_left_to_bottom_right(text, &patterns);
    sum += search_diagonal_bottom_left_to_top_right(&text, &patterns);

    return sum;
}

// ############################################################################################
// ############################################################################################
// # VARIANT 2                                                                                #
// ############################################################################################
// ############################################################################################

struct Coords {
    x: usize,
    y: usize,
}

fn three_by_three_window(text: &Vec<&str>, start: Coords) -> String {
    let mut three_by_three_window: Vec<String> = vec![String::new(); 3];
    let mut current_line = 0;

    for y in start.y..start.y + 3 {
        let line = text[y].to_string();
        three_by_three_window[current_line] = line[start.x..start.x + 3].to_string();
        current_line += 1;
    }
    let three_by_three_window = three_by_three_window.join("\n");
    return three_by_three_window.to_string();
}

pub fn x_mas_search(text: &str) -> u32 {
    let text: Vec<&str> = text.trim().split("\n").collect();

    let line_length = text.last().unwrap().chars().into_iter().count();
    let line_count = text.iter().count();

    let mut sum = 0;

    let patterns = (r"MAS", r"SAM");

    // move windows
    for window_y in 0..line_count - 2 {
        for window_x in 0..line_length - 2 {
            let window = three_by_three_window(
                &text,
                Coords {
                    x: window_x,
                    y: window_y,
                },
            );

            let found_diagonal_1 = search_diagonal_top_left_to_bottom_right(&window, &patterns) > 0;
            let found_diagonal_2 = search_diagonal_bottom_left_to_top_right(&window, &patterns) > 0;

            if found_diagonal_1 && found_diagonal_2 {
                sum += 1;
            }
        }
    }

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
        flip_text_by_45, flip_text_by_90, flip_text_by_minus_45, search_horizontally, x_mas_search,
        xmas_search,
    };

    #[test]
    fn test_x_mas_search() {
        let text = "OMOSOOOOOO
OOAOOMSMSO
OMOSOMAAOO
OOAOASMSMO
OMOSOMOOOO
OOOOOOOOOO
SOSOSOSOSO
OAOAOAOAOO
MOMOMOMOMO
OOOOOOOOOO";
        let xmas_count = x_mas_search(text);
        assert_eq!(xmas_count, 9);
    }

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
        let xmas_count = search_horizontally(text, &(r"XMAS", r"SAMX"));
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
