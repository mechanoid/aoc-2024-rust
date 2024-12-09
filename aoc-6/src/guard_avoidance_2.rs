use crate::guard_avoidance_1::{
    find_guard, perspective_char, predict_next_step, show_map, update_position, Location, Map,
    Perspective,
};

fn way_marker(perspective: Perspective) -> char {
    return match perspective {
        Perspective::Up => '|',
        Perspective::Down => '|',
        Perspective::Left => '-',
        Perspective::Right => '-',
    };
}

fn check_if_we_hit_the_obstacle_again(obstacles: &mut Vec<String>, obstacle: Location) -> bool {
    let (x, y, perspective) = obstacle;
    let original_perspective = perspective_char(&perspective);

    let key = format!("{}:{}:{}", x, y, original_perspective).to_string();

    if obstacles.contains(&key) {
        // yeah, we've found a loop
        return true;
    }

    if !obstacles.contains(&key) {
        // memorize obstacle, so we can recognize later if we hit it again
        obstacles.push(key);
        return false;
    }
    return true;
}

pub fn walk_and_check_for_loop(
    map: &Map,
    original_guards_path: &Map,
    obstacle_x: usize,
    obstacle_y: usize,
) -> bool {
    let mut map = map.clone();
    let mut obstacles: Vec<String> = vec![];
    let mut crossing: Option<(usize, usize)> = None;

    if original_guards_path[obstacle_y][obstacle_x] == 'X' {
        map[obstacle_y][obstacle_x] = 'O'; // place obstacle
    } else {
        // obstacles that are not in the original path of the guard do not change a thing, so let's skip them
        return false;
    }

    loop {
        let (before_location, after_location, original_target) = predict_next_step(&map);
        // let's get current location
        let (x, y, old_perspective) = before_location;
        //let's get the location, the guard approaches next

        if let Some((new_x, new_y, new_perspective)) = after_location {
            if let Some((cx, cy)) = crossing {
                map[cy][cx] = '+';
                crossing = None;
            };

            // in case of turns, we would have hit a different, original target. Let's memorize those,
            // to see if we hit one again. If so, we've found a loop!
            if let Some(original_target) = original_target {
                if check_if_we_hit_the_obstacle_again(&mut obstacles, original_target) {
                    return true;
                }
                // remember if we saw a crossing, so we can draw it in the next round
                // without removing the guard itself. Because in this round the guard is only turning around.
                crossing = Some((x, y));
            }

            // draw new guard position to the map
            update_position(&mut map, new_x, new_y, new_perspective);

            // when we are not redirected
            if let None = original_target {
                // draw way of the guard to the map
                map[y][x] = way_marker(old_perspective);
            }
        } else {
            // there is no guard anymore but we want to mark its last path
            map[y][x] = way_marker(old_perspective);
            break;
        }
    }

    return false;
}

pub fn find_loops(map: &Map, original_guards_path: &Map) -> u32 {
    let map_width = original_guards_path[0].len(); // horizontal
    let map_length = original_guards_path.len(); // vertical

    let (guard_start_x, guard_start_y, _) = find_guard(&map).unwrap();

    let mut loop_count: u32 = 0;
    let mut coords: Vec<(usize, usize)> = vec![];
    let mut checked: Vec<(usize, usize)> = vec![];

    for obstacle_y in 0..map_length {
        for obstacle_x in 0..map_width {
            println!(
                "line: {} / col: {}",
                format!("{:#03}", obstacle_y),
                format!("{:#03}", obstacle_x)
            );

            if obstacle_x == guard_start_x && obstacle_y == guard_start_y {
                continue; // don't try the guards start location for a new obstacle
            }

            if walk_and_check_for_loop(&map, &original_guards_path, obstacle_x, obstacle_y) {
                coords.push((obstacle_x, obstacle_y));
                loop_count += 1;
            } else {
                checked.push((obstacle_x, obstacle_y));
            }
        }
    }

    let mut map = original_guards_path.clone();

    for (x, y) in checked {
        if original_guards_path[y][x] == 'X' {
            map[y][x] = '█';
        }
    }

    for (x, y) in coords {
        map[y][x] = 'O';
    }

    println!("\n\n{}\n\n", show_map(&map));

    return loop_count;
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

        let original_guards_path = parse_map(
            "....#.....
             ....XXXXX#
             ....X...X.
             ..#.X...X.
             ..XXXXX#X.
             ..X.X.X.X.
             .#XXXXXXX.
             .XXXXXXX#.
             #XXXXXXX..
             ......#X..",
        );

        let loops = find_loops(&map, &original_guards_path);

        assert_eq!(loops, 6);
    }
}
