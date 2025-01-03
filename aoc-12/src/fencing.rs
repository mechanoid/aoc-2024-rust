type Map = Vec<Vec<GardenPlot>>;
type Region = Vec<GardenPlot>;

#[derive(Clone, Debug, PartialEq)]
pub struct GardenPlot {
    id: char,
    x: u64,
    y: u64,
    perimeters: u8,
    checked: bool,
}

fn plot(id: char, x: u64, y: u64, perimeters: u8) -> GardenPlot {
    return GardenPlot {
        id,
        x,
        y,
        perimeters,
        checked: false,
    };
}

pub fn parse(map: &str) -> Map {
    map.trim()
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| plot(c, x as u64, y as u64, 0))
                .collect::<Vec<GardenPlot>>()
        })
        .collect::<Map>()
}

fn find_perimeters(map: &Map, id: char, x: usize, y: usize) -> u8 {
    let mut perimeters = 0;

    // left border
    if x == 0 || map[y][x - 1].id != id {
        perimeters += 1;
    }

    // right border
    if x == map[y].len() - 1 || map[y][x + 1].id != id {
        perimeters += 1;
    }

    // top border
    if y == 0 || map[y - 1][x].id != id {
        perimeters += 1;
    }

    // top border
    if y == map.len() - 1 || map[y + 1][x].id != id {
        perimeters += 1;
    }

    return perimeters;
}

fn check_plot(map: &mut Map, region: &mut Region, id: char, x: usize, y: usize) {
    let current_plot = &map[y][x];

    // already checked or not part of that region
    if current_plot.checked || current_plot.id != id {
        return;
    }

    // // mark the plot as checked, so that we don't revisit it
    map[y][x].checked = true;

    let current_plot = &map[y][x]; // get updated plot reference
    let perimeters = find_perimeters(&map, id, x, y);
    let mut region_plot = current_plot.clone();
    region_plot.perimeters = perimeters;
    region.push(region_plot);

    // check plots to the left
    if x > 0 {
        check_plot(map, region, id, x - 1, y);
    }

    // check plots to the right
    if x < map[y].len() - 1 {
        check_plot(map, region, id, x + 1, y);
    }

    // check plots to the top
    if y > 0 {
        check_plot(map, region, id, x, y - 1);
    }

    // // check plots to the bottom
    if y < map.len() - 1 {
        check_plot(map, region, id, x, y + 1);
    }

    // check plots to the left
}

fn discover_region(map: &mut Map, id: char, x: usize, y: usize) -> Option<Region> {
    let mut region = vec![];
    check_plot(map, &mut region, id, x, y);

    if region.len() > 0 {
        return Some(region);
    }

    return None;
}

pub fn find_regions(map: &Map) -> Vec<Region> {
    let mut plan = map.clone();
    let mut regions: Vec<Region> = vec![];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let plot = &map[y][x];

            if let Some(region) = discover_region(&mut plan, plot.id, x, y) {
                regions.push(region);
            };
        }
    }

    return regions;
}

pub fn fencing_price(map: &Map) -> u64 {
    let mut price = 0;
    let regions = find_regions(map);

    for region in regions {
        let perimeters = region
            .iter()
            .map(|plot| plot.perimeters as u64)
            .reduce(|acc, perimeter| acc + perimeter)
            .unwrap();
        price += perimeters * region.len() as u64;
    }

    return price;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {

    use super::{fencing_price, find_perimeters, find_regions, parse};

    #[test]
    fn test_find_perimeters() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let perimeters = find_perimeters(&map, 'A', 0, 0);
        assert_eq!(perimeters, 3);

        let perimeters = find_perimeters(&map, 'A', 1, 0);
        assert_eq!(perimeters, 2);

        let perimeters = find_perimeters(&map, 'A', 2, 0);
        assert_eq!(perimeters, 2);

        let perimeters = find_perimeters(&map, 'A', 3, 0);
        assert_eq!(perimeters, 3);

        let perimeters = find_perimeters(&map, 'B', 1, 1);
        assert_eq!(perimeters, 2);

        let perimeters = find_perimeters(&map, 'D', 1, 3);
        assert_eq!(perimeters, 4);
    }
    #[test]
    fn test_regions() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let regions = find_regions(&map);

        assert_eq!(regions.len(), 5);

        assert_eq!(regions[0].len(), 4);
        assert_eq!(regions[1].len(), 4);
        assert_eq!(regions[2].len(), 4);
        assert_eq!(regions[3].len(), 1);
        assert_eq!(regions[4].len(), 3);
    }

    #[test]
    fn test_fencing_price() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let result = fencing_price(&map);

        assert_eq!(result, 140);

        let map = parse(
            "RRRRIICCFF
                  RRRRIICCCF
                  VVRRRCCFFF
                  VVRCCCJFFF
                  VVVVCJJCFE
                  VVIVCCJJEE
                  VVIIICJJEE
                  MIIIIIJJEE
                  MIIISIJEEE
                  MMMISSJEEE",
        );

        let result = fencing_price(&map);

        assert_eq!(result, 1930);
    }
}
