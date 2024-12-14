mod util;

pub use util::parse;
use util::{draw_antinodes, unique_chars};

pub type AntennaMap = Vec<Vec<char>>;
pub type Location = (i64, i64);
pub type AntennaLocation = (i64, i64, char);
pub type Vector = (i64, i64);

fn find_frequencies(map: &AntennaMap) -> Vec<char> {
    return unique_chars(&map);
}

fn locate_antennas(map: &AntennaMap, frequency: &char) -> Vec<AntennaLocation> {
    let mut locations: Vec<AntennaLocation> = vec![];

    for (line_index, line) in map.iter().enumerate() {
        for (location_index, location) in line.iter().enumerate() {
            if location == frequency {
                locations.push((location_index as i64, line_index as i64, frequency.clone()));
            }
        }
    }
    return locations;
}

fn antenna_connections(
    locations: &Vec<AntennaLocation>,
) -> Vec<(Vector, AntennaLocation, AntennaLocation)> {
    let mut connections: Vec<(Vector, AntennaLocation, AntennaLocation)> = vec![];

    for i in 0..locations.len() {
        for j in i..locations.len() {
            let (x1, y1, c1) = locations[i].clone();
            let (x2, y2, c2) = locations[j].clone();

            let distance_x = x2 - x1;
            let distance_y = y2 - y1;

            if distance_x == 0 && distance_y == 0 {
                continue;
            }

            connections.push(((distance_x, distance_y), (x1, y1, c1), (x2, y2, c2)));
        }
    }

    return connections;
}

fn valid(map: &AntennaMap, (x, y): Location) -> Result<Location, String> {
    let width = map[0].len() as i64;
    let height = map.len() as i64;

    if x >= 0 && x < width - 1 && y >= 0 && y < height - 1 && map[y as usize][x as usize] == '.' {
        return Ok((x, y).clone());
    }

    return Err("location is out of bounds or an antenna itself".to_string());
}

pub fn find_antinodes(map: AntennaMap) -> Result<u64, String> {
    let frequencies = find_frequencies(&map);
    let mut antinodes: Vec<Location> = vec![];

    for frequency in frequencies {
        let antennas = locate_antennas(&map, &frequency);
        let connections = antenna_connections(&antennas);

        for ((dist_x, dist_y), (x1, y1, _), (x2, y2, _)) in connections {
            let antinode_1 = (x1 - dist_x, y1 - dist_y);
            if let Ok(antinode_1) = valid(&map, antinode_1) {
                antinodes.push(antinode_1);
            };

            let antinode_2 = (x2 + dist_x, y2 + dist_y);
            if let Ok(antinode_2) = valid(&map, antinode_2) {
                antinodes.push(antinode_2);
            };
        }
    }
    draw_antinodes(&map, &antinodes);
    return Ok(antinodes.len() as u64);
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use super::{antenna_connections, find_antinodes, find_frequencies, locate_antennas, parse};

    #[test]
    fn test_antenna_connections() {
        let locations = vec![(7, 0, 'A'), (4, 1, 'A')];
        let result = antenna_connections(&locations);
        assert_eq!(result, vec![((-3, 1), (7, 0, 'A'), (4, 1, 'A'))]);

        let locations = vec![(5, 0, 'A'), (3, 2, 'A')];
        let result = antenna_connections(&locations);
        assert_eq!(result, vec![((-2, 2), (5, 0, 'A'), (3, 2, 'A'))]);

        let locations = vec![(5, 0, 'A'), (3, 2, 'A'), (4, 4, 'A')];
        let result = antenna_connections(&locations);
        assert_eq!(
            result,
            vec![
                ((-2, 2), (5, 0, 'A'), (3, 2, 'A')),
                ((-1, 4), (5, 0, 'A'), (4, 4, 'A')),
                ((1, 2), (3, 2, 'A'), (4, 4, 'A'))
            ]
        );
    }

    #[test]
    fn test_locate_antennas() {
        let antenna_map = parse(
            "
             .......A....
             ....A.......
             ......A.....",
        );

        let result = locate_antennas(&antenna_map, &'A');

        assert_eq!(result, vec![(7, 0, 'A'), (4, 1, 'A'), (6, 2, 'A')]);
    }

    #[test]
    fn test_find_frequencies() {
        let antenna_map = parse(
            "............
             ........0...
             .....0......
             .......0....
             ....0.......
             ......A.....
             ............
             ............
             ........A...
             .........A..
             ............
             ............",
        );

        let result = find_frequencies(&antenna_map);

        assert_eq!(result, vec!['0', 'A']);
    }

    #[test]
    fn test_find_antinodes() {
        let antenna_map = parse(
            "............
             ........0...
             .....0......
             .......0....
             ....0.......
             ......A.....
             ............
             ............
             ........A...
             .........A..
             ............
             ............",
        );

        let result = find_antinodes(antenna_map);

        assert_eq!(result, Ok(14));
    }
}
