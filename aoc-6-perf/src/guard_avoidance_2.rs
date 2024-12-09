use crate::guard_avoidance_1::{find_guard, perspective_char, predict_next_step, Location, Map};

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
    steps: &Vec<(usize, usize)>,
    obstacle_x: usize,
    obstacle_y: usize,
    initial_location: Location,
) -> bool {
    let mut map = map.clone();
    let mut obstacles: Vec<String> = vec![];
    let mut current_location = initial_location;

    if steps.contains(&(obstacle_x, obstacle_y)) {
        map[obstacle_y][obstacle_x] = 'O'; // place obstacle
    } else {
        // obstacles that are not in the original path of the guard do not change a thing, so let's skip them
        return false;
    }

    loop {
        let (next_location, original_target) = predict_next_step(&map, &current_location);

        if let Some(next_location) = next_location {
            current_location = next_location;
        } else {
            return false;
        }

        // in case of turns, we would have hit a different, original target. Let's memorize those,
        // to see if we hit one again. If so, we've found a loop!
        if let Some(original_target) = original_target {
            if check_if_we_hit_the_obstacle_again(&mut obstacles, original_target) {
                return true;
            }
        }
    }
}

pub fn find_loops(map: &Map, steps: Vec<(usize, usize)>) -> u32 {
    let steps = steps.clone();
    let map_width = map[0].len(); // horizontal
    let map_length = map.len(); // vertical

    let initial_location = find_guard(&map).unwrap();
    let (guard_start_x, guard_start_y, _) = &initial_location;

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
            if obstacle_x == *guard_start_x && obstacle_y == *guard_start_y {
                continue; // don't try the guards start location for a new obstacle
            }

            if walk_and_check_for_loop(&map, &steps, obstacle_x, obstacle_y, initial_location) {
                coords.push((obstacle_x, obstacle_y));
                loop_count += 1;
            } else {
                checked.push((obstacle_x, obstacle_y));
            }
        }
    }

    return loop_count;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use crate::{
        guard_avoidance_1::{parse_map, predict_path_positions},
        guard_avoidance_2::find_loops,
    };

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
        let (_, steps) = predict_path_positions(&map);
        let loops = find_loops(&map, steps);

        assert_eq!(loops, 6);
    }
}
