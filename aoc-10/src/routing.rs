type TopgraphicLevel = i8;
type TopographicMap = Vec<Vec<TopgraphicLevel>>;
type Position = (usize, usize);

pub fn parse(map: &str) -> TopographicMap {
    map.trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse::<TopgraphicLevel>().unwrap_or(-1))
                .collect::<Vec<TopgraphicLevel>>()
        })
        .collect::<TopographicMap>()
}

fn walkable(current_level: TopgraphicLevel, level: TopgraphicLevel) -> bool {
    if level == current_level + 1 {
        return true;
    }

    return false;
}

fn trail_head_targets(map: &TopographicMap, x: usize, y: usize) -> Vec<Position> {
    let mut available_routes: Vec<Position> = vec![];
    let current_level = map[y][x];

    if current_level == 9 {
        available_routes.push((x, y));
        return available_routes;
    }

    // walk right
    if map[y].len() - 1 > x && walkable(current_level, map[y][x + 1]) {
        available_routes = [available_routes, trail_head_targets(map, x + 1, y)].concat();
    }

    // walk left
    if x > 0 && walkable(current_level, map[y][x - 1]) {
        available_routes = [available_routes, trail_head_targets(map, x - 1, y)].concat();
    }

    // walk top
    if y > 0 && walkable(current_level, map[y - 1][x]) {
        available_routes = [available_routes, trail_head_targets(map, x, y - 1)].concat();
    }

    // walk bottom
    if y < map.len() - 1 && walkable(current_level, map[y + 1][x]) {
        available_routes = [available_routes, trail_head_targets(map, x, y + 1)].concat();
    }

    return available_routes;
}

pub fn trail_head_scoring(map: &TopographicMap) -> u32 {
    let mut paths_count = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 0 {
                continue;
            }

            let mut trail_head_targets = trail_head_targets(map, x, y);

            trail_head_targets.sort();
            trail_head_targets.dedup();

            let trail_head_score = trail_head_targets.len() as u32;
            paths_count += trail_head_score;
        }
    }

    return paths_count;
}

pub fn trail_head_rating(map: &TopographicMap) -> u32 {
    let mut paths_count = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 0 {
                continue;
            }

            let trail_head_targets = trail_head_targets(map, x, y);
            let trail_head_score = trail_head_targets.len() as u32;
            paths_count += trail_head_score;
        }
    }

    return paths_count;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use super::{parse, trail_head_rating, trail_head_scoring, trail_head_targets, walkable};

    #[test]
    fn test_walkable() {
        assert_eq!(walkable(0, 1), true);
        assert_eq!(walkable(3, 4), true);
        assert_eq!(walkable(3, 5), false);
        assert_eq!(walkable(1, 9), false);
    }

    #[test]
    fn test_trail_head_targets() {
        let map = parse(
            "...0...
                  ...1...
                  ...2...
                  6543456
                  7.....7
                  8.....8
                  9.....9
                  ",
        );

        let result = trail_head_targets(&map, 3, 0);
        assert_eq!(result.len(), 2);

        let map = parse(
            "..90..9
                  ...1.98
                  ...2..7
                  6543456
                  765.987
                  876....
                  987....",
        );

        let result = trail_head_targets(&map, 3, 0);
        assert_eq!(result.len(), 13);
    }

    #[test]
    fn test_trail_head_scoring() {
        let map = parse(
            "10..9..
                  2...8..
                  3...7..
                  4567654
                  ...8..3
                  ...9..2
                  .....01
",
        );
        let result = trail_head_scoring(&map);
        assert_eq!(result, 3);

        let map = parse(
            "89010123
                 78121874
                 87430965
                 96549874
                 45678903
                 32019012
                 01329801
                 10456732",
        );
        let result = trail_head_scoring(&map);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_trail_head_rating() {
        let map = parse(
            "..90..9
                  ...1.98
                  ...2..7
                  6543456
                  765.987
                  876....
                  987....",
        );
        let result = trail_head_rating(&map);
        assert_eq!(result, 13);

        let map = parse(
            "012345
123456
234567
345678
4.6789
56789.",
        );
        let result = trail_head_rating(&map);
        assert_eq!(result, 227);

        let map = parse(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732.",
        );
        let result = trail_head_rating(&map);
        assert_eq!(result, 81);
    }
}
