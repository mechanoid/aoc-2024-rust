#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Perspective {
    Up,
    Down,
    Left,
    Right,
}
pub fn perspective_char(perspective: &Perspective) -> char {
    return match perspective {
        Perspective::Up => '^',
        Perspective::Down => 'v',
        Perspective::Left => '<',
        Perspective::Right => '>',
    };
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

pub fn find_guard(map: &Map) -> Option<Location> {
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

fn go_up(map: &Map, location: &Location) -> (Option<Location>, Option<Location>) {
    let (x, y, perspective) = location.clone();

    if y > 0 {
        let new_y = y - 1;
        let next_field = map[new_y][x];

        if next_field == '#' || next_field == 'O' {
            // let original_target = next_field;
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some((x, new_y, perspective)));
        } else {
            return (Some((x, new_y, perspective)), None);
        }
    }

    return (None, None);
}

fn go_down(map: &Map, location: &Location) -> (Option<Location>, Option<Location>) {
    let (x, y, perspective) = location.clone();

    if y < map.len() - 1 {
        let new_y = y + 1;
        let next_field = map[new_y][x];

        if next_field == '#' || next_field == 'O' {
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some((x, new_y, perspective)));
        } else {
            return (Some((x, new_y, perspective)), None);
        }
    }

    return (None, None);
}

fn go_right(map: &Map, location: &Location) -> (Option<Location>, Option<Location>) {
    let (x, y, perspective) = location.clone();
    let first_line = map.first().unwrap();

    if x < first_line.len() - 1 {
        let new_x = x + 1;

        let next_field = map[y][new_x];

        if next_field == '#' || next_field == 'O' {
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some((new_x, y, perspective)));
        } else {
            return (Some((new_x, y, perspective)), None);
        }
    } else {
        return (None, None); // left the Map
    }
}

fn go_left(map: &Map, location: &Location) -> (Option<Location>, Option<Location>) {
    let (x, y, perspective) = location.clone();

    if x > 0 {
        let new_x = x - 1;

        let next_field = map[y][new_x];

        if next_field == '#' || next_field == 'O' {
            let new_perspective = rotate(&perspective);
            return (Some((x, y, new_perspective)), Some((new_x, y, perspective)));
        } else {
            return (Some((new_x, y, perspective)), None);
        }
    }

    return (None, None);
}

pub fn predict_next_step(
    map: &Map,
    current_location: &Location,
) -> (Option<Location>, Option<Location>) {
    return match &current_location.2 {
        Perspective::Up => go_up(&map, &current_location),
        Perspective::Right => go_right(&map, &current_location),
        Perspective::Down => go_down(&map, &current_location),
        Perspective::Left => go_left(&map, &current_location),
    };
}

pub fn predict_path_positions<'b>(map: &Map) -> (usize, Vec<(usize, usize)>) {
    if let Some(initial_location) = find_guard(&map) {
        let mut steps: Vec<(usize, usize)> = vec![];
        let mut current_location = initial_location;

        loop {
            let (x, y, perspective) = current_location;
            let (next_location, _) = predict_next_step(&map, &current_location);

            if let Some((new_x, new_y, new_perspective)) = next_location {
                current_location = (new_x, new_y, new_perspective);

                // check if guard has moved at all (or just turned around)
                if perspective == new_perspective && (x != new_x || y != new_y) {
                    let coords = (new_x, new_y);
                    if !steps.contains(&coords) {
                        steps.push(coords);
                    }
                }
            } else {
                // moved out of map
                break;
            }
        }
        return (steps.len(), steps);
    }

    panic!("no guard found!");
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
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((4, 0, Perspective::Up)));
    }

    #[test]
    fn test_going_up_out_of_map() {
        let map = parse_map("...^#....."); // leaving map
        let initial_location = find_guard(&map).unwrap();
        let (after_locatoon, _) = predict_next_step(&map, &initial_location);
        assert_eq!(after_locatoon, None);
    }

    #[test]
    fn test_going_up_against_obstacle() {
        let map = parse_map(
            "....#.....
             .#..^.....",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((4, 1, Perspective::Right)));
    }

    #[test]
    fn test_going_right() {
        let map = parse_map(
            "....#.....
             .#..>.#...",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((5, 1, Perspective::Right)));
    }

    #[test]
    fn test_going_right_against_obstacle() {
        let map = parse_map(
            "....#.....
             .#..>#....",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((4, 1, Perspective::Down)));
    }

    #[test]
    fn test_going_right_out_of_map() {
        let map = parse_map(".#...#...>"); // leaving map
        let initial_location = find_guard(&map).unwrap();
        let (after_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(after_location, None);
    }

    #[test]
    fn test_going_down() {
        let map = parse_map(
            "....#.v...
             .#...#....",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((6, 1, Perspective::Down)));
    }

    #[test]
    fn test_going_down_against_obstacle() {
        let map = parse_map(
            "....#.v...
             .#....#...",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((6, 0, Perspective::Left)));
    }

    #[test]
    fn test_going_down_leaving_the_map() {
        let map = parse_map("....#.v...");
        let initial_location = find_guard(&map).unwrap();
        let (after_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(after_location, None);
    }

    #[test]
    fn test_going_left() {
        let map = parse_map(
            "....#.....
             .#.<.#....",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((2, 1, Perspective::Left)));
    }

    #[test]
    fn test_going_left_against_obstacle() {
        let map = parse_map(
            "....#.....
             .#<..#....",
        );
        let initial_location = find_guard(&map).unwrap();
        let (next_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(next_location, Some((2, 1, Perspective::Up)));
    }

    #[test]
    fn test_going_left_out_of_map() {
        let map = parse_map(
            "....#.....
             <....#....",
        );
        let initial_location = find_guard(&map).unwrap();
        let (after_location, _) = predict_next_step(&map, &initial_location);
        assert_eq!(after_location, None);
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
    fn test_predict_direct_path_out() {
        let map = parse_map(
            "..........
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
        let (steps, _) = predict_path_positions(&map);

        assert_eq!(steps, 6);
    }

    #[test]
    fn test_predict_one_uturn() {
        let map = parse_map(
            "....#.....
             ......#...
             .#..^.....",
        );
        let (steps, _) = predict_path_positions(&map);

        assert_eq!(steps, 3);
    }

    #[test]
    fn test_predict_two_turns() {
        let map = parse_map(
            "....#.....
             ........#.
             .#..^.....",
        );
        let (steps, _) = predict_path_positions(&map);

        assert_eq!(steps, 5);
    }

    #[test]
    fn test_predict_multiple_turns() {
        let map = parse_map(
            "....#.....
             ........#.
             .#..^.....
             .#.....#..",
        );
        let (steps, _) = predict_path_positions(&map);

        assert_eq!(steps, 12);
    }

    #[test]
    fn test_predict_path() {
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
        let (steps, _) = predict_path_positions(&map);

        assert_eq!(steps, 41);
    }
}
