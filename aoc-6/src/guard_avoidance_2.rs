use crate::guard_avoidance_1::{predict_next_step, show_map, update_position, Map, Perspective};

fn way_marker(perspective: Perspective) -> char {
    return match perspective {
        Perspective::Up => '|',
        Perspective::Down => '|',
        Perspective::Left => '-',
        Perspective::Right => '-',
    };
}

pub fn find_loops(map: &Map) -> String {
    let mut map = map.clone();

    while let Some((before_location, after_location, _original_target)) = predict_next_step(&map) {
        if let Some((x, y, old_perspective)) = before_location {
            if let Some((new_x, new_y, new_perspective)) = after_location {
                update_position(&mut map, new_x, new_y, new_perspective);

                if old_perspective != new_perspective {
                    if let (_, Some((next_x, next_y, _)), _) = predict_next_step(&map).unwrap() {
                        update_position(&mut map, next_x, next_y, new_perspective);
                        map[y][x] = '+';
                    };
                    continue;
                } else {
                    map[y][x] = way_marker(old_perspective);
                }
            } else {
                map[y][x] = way_marker(old_perspective);
            }
        } else {
            panic!("failed to resolve before location!")
        }
    }

    let rendered_map = show_map(&map);
    println!("{:?}", rendered_map);
    return rendered_map;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use crate::{guard_avoidance_1::parse_map, guard_avoidance_2::find_loops};

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

        let map_after = find_loops(&map);

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
    }
}
