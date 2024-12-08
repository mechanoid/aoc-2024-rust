use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Perspective {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Perspective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Perspective::Up => write!(f, "^"),
            Perspective::Down => write!(f, "v"),
            Perspective::Left => write!(f, "<"),
            Perspective::Right => write!(f, ">"),
        }
    }
}

pub type Location = (usize, usize, Perspective);
pub type Map<'a> = Vec<Vec<char>>;

pub fn parse_map(map: &str) -> Map {
    let map = map.trim();
    let map: Vec<&str> = map.split("\n").collect();
    let map: Vec<Vec<char>> = map
        .iter()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();
    return map;
}

fn perspective(c: &char) -> Result<Perspective, String> {
    return match c {
        '^' => Ok(Perspective::Up),
        '<' => Ok(Perspective::Left),
        '>' => Ok(Perspective::Right),
        'v' => Ok(Perspective::Down),
        _ => Err("what direction is this guard even looking at??（◎ . ◎）".to_string()),
    };
}

fn rotate(perspective: &Perspective) -> Perspective {
    return match perspective {
        Perspective::Up => Perspective::Right,
        Perspective::Right => Perspective::Down,
        Perspective::Down => Perspective::Left,
        Perspective::Left => Perspective::Up,
    };
}

fn find_guard(map: &Map) -> Option<Location> {
    let guard_shapes = ['^', '<', '>', 'v'];

    for (y, line) in map.iter().enumerate() {
        let line = line.clone();
        let line = line.iter().collect::<Vec<&char>>();

        if let Some(x) = line.iter().position(|c| guard_shapes.contains(c)) {
            let guard = line[x];
            let perspective = perspective(&guard).unwrap();

            return Some((x, y, perspective));
        }
    }

    return None;
}

fn go_up(map: &Map, location: &Location) -> (Option<Location>, Option<char>) {
    let (x, y, perspective) = location.clone();

    if y > 0 {
        let new_y = y - 1;
        let next_field = map[new_y][x];

        if next_field == '#' || next_field == 'O' {
            let original_target = next_field;
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some(original_target));
        } else {
            return (Some((x, new_y, perspective)), None);
        }
    }

    return (None, None);
}

fn go_down(map: &Map, location: &Location) -> (Option<Location>, Option<char>) {
    let (x, y, perspective) = location.clone();

    if y < map.len() - 1 {
        let new_y = y + 1;
        let next_field = map[new_y][x];

        if next_field == '#' || next_field == 'O' {
            let original_target = next_field;
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some(original_target));
        } else {
            return (Some((x, new_y, perspective)), None);
        }
    }

    return (None, None);
}

fn go_right(map: &Map, location: &Location) -> (Option<Location>, Option<char>) {
    let (x, y, perspective) = location.clone();
    let first_line = map.first().unwrap();

    if x < first_line.len() - 1 {
        let new_x = x + 1;

        let next_field = map[y][new_x];

        if next_field == '#' || next_field == 'O' {
            let original_target = next_field;
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some(original_target));
        } else {
            return (Some((new_x, y, perspective)), None);
        }
    }

    return (None, None);
}

fn go_left(map: &Map, location: &Location) -> (Option<Location>, Option<char>) {
    let (x, y, perspective) = location.clone();

    if x > 0 {
        let new_x = x - 1;

        let next_field = map[y][new_x];

        if next_field == '#' || next_field == 'O' {
            let original_target = next_field;
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some(original_target));
        } else {
            return (Some((new_x, y, perspective)), None);
        }
    }

    return (None, None);
}

pub fn predict_next_step(map: &Map) -> Option<(Option<Location>, Option<Location>, Option<char>)> {
    if let Some(location) = find_guard(&map) {
        let (_, _, perspective) = location.clone();

        let (next_location, original_target) = match &perspective {
            Perspective::Up => go_up(&map, &location),
            Perspective::Right => go_right(&map, &location),
            Perspective::Down => go_down(&map, &location),
            Perspective::Left => go_left(&map, &location),
        };

        return Some((Some(location), next_location, original_target));
    }

    return None;
}

fn mark_last_position(map: &mut Map, x: usize, y: usize) {
    map[y][x] = 'X';
}

pub fn update_position(map: &mut Map, x: usize, y: usize, new_perspective: Perspective) {
    map[y][x] = new_perspective.to_string().chars().next().unwrap()
}

pub fn show_map(map: &Map) -> String {
    let map = &map
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");
    return map.to_string();
}

pub fn predict_path_positions(map: &mut str) -> (Map, String, usize) {
    let mut map: Map = parse_map(&map);

    while let Some((before_location, after_location, _)) = predict_next_step(&map) {
        // update_map(&mut map, &before_location, &after_location);
        if let Some(before_location) = before_location {
            let (x, y, _) = before_location;
            mark_last_position(&mut map, x, y);

            if let Some(after_location) = after_location {
                let (x, y, new_perspective) = after_location;
                update_position(&mut map, x, y, new_perspective);
            }
        }
    }

    let rendered_map = show_map(&map);
    let steps = &rendered_map.chars().filter(|c| *c == 'X').count();
    return (map, rendered_map, *steps);
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use super::{
        find_guard, parse_map, perspective, predict_next_step, predict_path_positions, Perspective,
    };

    #[test]
    fn test_going_up() {
        let map = parse_map(
            ".....#....
             .#..^.....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((4, 0, Perspective::Up)));
    }

    #[test]
    fn test_going_up_out_of_map() {
        let map = parse_map("...^#....."); // leaving map
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, None);
    }

    #[test]
    fn test_going_up_against_obstacle() {
        let map = parse_map(
            "....#.....
             .#..^.....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((4, 1, Perspective::Right)));
    }

    #[test]
    fn test_going_right() {
        let map = parse_map(
            "....#.....
             .#..>.#...",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((5, 1, Perspective::Right)));
    }

    #[test]
    fn test_going_right_against_obstacle() {
        let map = parse_map(
            "....#.....
             .#..>#....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((4, 1, Perspective::Down)));
    }

    #[test]
    fn test_going_right_out_of_map() {
        let map = parse_map(".#...#...>"); // leaving map
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, None);
    }

    #[test]
    fn test_going_down() {
        let map = parse_map(
            "....#.v...
             .#...#....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((6, 1, Perspective::Down)));
    }

    #[test]
    fn test_going_down_against_obstacle() {
        let map = parse_map(
            "....#.v...
             .#....#...",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((6, 0, Perspective::Left)));
    }

    #[test]
    fn test_going_down_leaving_the_map() {
        let map = parse_map("....#.v...");
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, None);
    }

    #[test]
    fn test_going_left() {
        let map = parse_map(
            "....#.....
             .#.<.#....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((2, 1, Perspective::Left)));
    }

    #[test]
    fn test_going_left_against_obstacle() {
        let map = parse_map(
            "....#.....
             .#<..#....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, Some((2, 1, Perspective::Up)));
    }

    #[test]
    fn test_going_left_out_of_map() {
        let map = parse_map(
            "....#.....
             <....#....",
        );
        let (_, next_location, _) = predict_next_step(&map).unwrap();
        assert_eq!(next_location, None);
    }

    #[test]
    fn test_find_guard() {
        let map = parse_map(
            "....#.....
             .........#
             ..........
             ..#.......
             .......#..
             ..........
             .#..^.....
             ........#.
             #.........
             ......#...",
        );

        let guard_position = find_guard(&map);
        assert_eq!(guard_position, Some((4, 6, Perspective::Up)))
    }

    #[test]
    fn test_perspective() {
        let g = '^';
        let p = perspective(&g);
        assert_eq!(p, Ok(Perspective::Up));

        let g = 'v';
        let p = perspective(&g);
        assert_eq!(p, Ok(Perspective::Down));

        let g = '<';
        let p = perspective(&g);
        assert_eq!(p, Ok(Perspective::Left));

        let g = '>';
        let p = perspective(&g);
        assert_eq!(p, Ok(Perspective::Right));
    }

    #[test]
    fn test_predict_path() {
        let mut map = "....#.....
                             .........#
                             ..........
                             ..#.......
                             .......#..
                             ..........
                             .#..^.....
                             ........#.
                             #.........
                             ......#..."
            .to_string();

        let (_, map_after, steps) = predict_path_positions(&mut map);

        assert_eq!(
            map_after,
            "....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X.."
        );

        assert_eq!(steps, 41);
    }
}
