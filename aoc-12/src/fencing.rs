type Map = Vec<Vec<GardenPlot>>;
type Region = Vec<GardenPlot>;
type Coord = usize;

#[derive(Clone, Debug, PartialEq)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
struct Site {
    edge: Edge,
    from: GardenPlot,
    to: GardenPlot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GardenPlot {
    id: char,
    x: Coord,
    y: Coord,
    perimeters: Vec<Edge>,
    checked: bool,
}

fn plot(id: char, x: Coord, y: Coord, perimeters: Vec<Edge>) -> GardenPlot {
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
                .map(|(x, c)| plot(c, x as Coord, y as Coord, vec![]))
                .collect::<Vec<GardenPlot>>()
        })
        .collect::<Map>()
}

fn find_perimeters(map: &Map, id: char, x: Coord, y: Coord) -> Vec<Edge> {
    let mut perimeters: Vec<Edge> = vec![];

    // left border
    if x == 0 || map[y][x - 1].id != id {
        perimeters.push(Edge::Left);
    }

    // right border
    if x == map[y].len() - 1 || map[y][x + 1].id != id {
        perimeters.push(Edge::Right);
    }

    // top border
    if y == 0 || map[y - 1][x].id != id {
        perimeters.push(Edge::Top);
    }

    // top border
    if y == map.len() - 1 || map[y + 1][x].id != id {
        perimeters.push(Edge::Bottom);
    }

    return perimeters;
}

fn check_plot(map: &mut Map, region: &mut Region, id: char, x: Coord, y: Coord) {
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
    if usize::from(y) < map.len() - 1 {
        check_plot(map, region, id, x, y + 1);
    }

    // check plots to the left
}

fn discover_region(map: &mut Map, id: char, x: Coord, y: Coord) -> Option<Region> {
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

            if let Some(region) = discover_region(&mut plan, plot.id, x as Coord, y as Coord) {
                regions.push(region);
            };
        }
    }

    return regions;
}

fn find_plot_in_region(region: &Region, x: usize, y: usize) -> Option<GardenPlot> {
    if let Some(plot) = region.iter().find(|plot| plot.y == y && plot.x == x) {
        return Some(plot.clone());
    }

    return None;
}

fn follow<FX, FY>(
    edge: &Edge,
    region: &Region,
    plot_to_check: &GardenPlot,
    update_x: FX,
    update_y: FY,
) -> Option<GardenPlot>
where
    FX: Fn(i32) -> i32,
    FY: Fn(i32) -> i32,
{
    let GardenPlot {
        x, y, perimeters, ..
    } = plot_to_check;

    // only if current plot has the edge we search for it is a candidate for ending the site.
    // Otherwise it is always None.
    if perimeters.contains(&edge) {
        let new_x = update_x(*x as i32);
        let new_y = update_y(*y as i32);

        // We are out of the usize boundary, with negative numbers and we don't need to search further.
        // The upper boundaries we don't need to check, because they just cannot be found and result in none later.
        if new_x < 0 || new_y < 0 {
            return Some(plot_to_check.clone());
        }

        // let's see if there is a candidate plot next to the current slot
        if let Some(next_plot) = find_plot_in_region(region, new_x as usize, new_y as usize) {
            if let Some(new_plot) = follow(edge, region, &next_plot, update_x, update_y) {
                return Some(new_plot);
            };
        }

        return Some(plot_to_check.clone());
    }

    return None;
}

fn find_horizontal_site(edge: &Edge, region: &Region, plot: &GardenPlot) -> Option<Site> {
    if let Some(from) = follow(edge, region, plot, |x| x - 1, |y| y) {
        if let Some(to) = follow(edge, region, plot, |x| x + 1, |y| y) {
            return Some(Site {
                edge: edge.clone(),
                from,
                to,
            });
        }
    }

    return None;
}

fn find_vertical_site(edge: &Edge, region: &Region, plot: &GardenPlot) -> Option<Site> {
    if let Some(from) = follow(edge, region, plot, |x| x, |y| y - 1) {
        if let Some(to) = follow(edge, region, plot, |x| x, |y| y + 1) {
            return Some(Site {
                edge: edge.clone(),
                from,
                to,
            });
        }
    }

    return None;
}

fn find_sites(region: &Region) -> u16 {
    let mut sites: Vec<Site> = vec![];

    for plot in region {
        if let Some(site) = find_horizontal_site(&Edge::Top, region, plot) {
            if !sites.contains(&site) {
                sites.push(site);
            }
        };

        if let Some(site) = find_horizontal_site(&Edge::Bottom, region, plot) {
            if !sites.contains(&site) {
                sites.push(site);
            }
        };

        if let Some(site) = find_vertical_site(&Edge::Left, region, plot) {
            if !sites.contains(&site) {
                sites.push(site);
            }
        };

        if let Some(site) = find_vertical_site(&Edge::Right, region, plot) {
            if !sites.contains(&site) {
                sites.push(site);
            }
        };
    }

    return sites.len() as u16;
}

pub fn fencing_price(map: &Map) -> u64 {
    let mut price = 0;
    let regions = find_regions(map);

    for region in regions {
        let perimeters = region
            .iter()
            .map(|plot| plot.perimeters.len() as u64)
            .reduce(|acc, perimeter| acc + perimeter)
            .unwrap();
        price += perimeters * region.len() as u64;
    }

    return price;
}

pub fn bulk_fencing_price(map: &Map) -> u64 {
    let mut price = 0;
    let regions = find_regions(map);

    for region in regions {
        let sites = find_sites(&region);
        price += sites as u64 * region.len() as u64;
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

    use super::{
        bulk_fencing_price, fencing_price, find_horizontal_site, find_perimeters, find_regions,
        find_sites, parse, Edge, Site,
    };

    #[test]
    fn test_find_horizontal_site() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let regions = find_regions(&map);

        let region = &regions[0];
        let site = find_horizontal_site(&Edge::Top, &region, &region[0]);
        let Site { from, to, edge } = site.unwrap();
        assert_eq!(edge, Edge::Top);
        assert_eq!((from.x, from.y), (0, 0));
        assert_eq!((to.x, to.y), (3, 0));

        let region = &regions[2];
        let site = find_horizontal_site(&Edge::Top, &region, &region[2]);
        let Site { from, to, edge } = site.unwrap();
        assert_eq!(edge, Edge::Top);
        assert_eq!((from.x, from.y), (3, 2));
        assert_eq!((to.x, to.y), (3, 2));
    }

    #[test]
    fn test_find_sites() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let regions = find_regions(&map);

        let region = &regions[0];
        let sites = find_sites(&region);
        assert_eq!(sites, 4);

        let region = &regions[2];
        let sites = find_sites(&region);
        assert_eq!(sites, 8);
    }

    #[test]
    fn test_find_perimeters() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let perimeters = find_perimeters(&map, 'A', 0, 0);
        assert_eq!(perimeters, vec![Edge::Left, Edge::Top, Edge::Bottom]);

        let perimeters = find_perimeters(&map, 'A', 1, 0);
        assert_eq!(perimeters, vec![Edge::Top, Edge::Bottom]);

        let perimeters = find_perimeters(&map, 'A', 2, 0);
        assert_eq!(perimeters, vec![Edge::Top, Edge::Bottom]);

        let perimeters = find_perimeters(&map, 'A', 3, 0);
        assert_eq!(perimeters, vec![Edge::Right, Edge::Top, Edge::Bottom]);

        let perimeters = find_perimeters(&map, 'B', 1, 1);
        assert_eq!(perimeters, vec![Edge::Right, Edge::Top]);

        let perimeters = find_perimeters(&map, 'D', 1, 3);
        assert_eq!(perimeters.len(), 4);
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

    #[test]
    fn test_bulk_fencing_price() {
        let map = parse(
            "AAAA
                  BBCD
                  BBCC
                  EEEC",
        );

        let result = bulk_fencing_price(&map);

        assert_eq!(result, 80);

        let map = parse(
            "OOOOO
                  OXOXO
                  OOOOO
                  OXOXO
                  OOOOO",
        );

        let result = bulk_fencing_price(&map);

        assert_eq!(result, 436);

        let map = parse(
            "EEEEE
                  EXXXX
                  EEEEE
                  EXXXX
                  EEEEE",
        );

        let result = bulk_fencing_price(&map);

        assert_eq!(result, 236);
    }
}
