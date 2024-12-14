use std::collections::HashSet;

use super::{AntennaMap, Location};

pub fn parse(antenna_map: &str) -> AntennaMap {
    return antenna_map
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<AntennaMap>();
}

pub fn unique_chars(map: &AntennaMap) -> Vec<char> {
    let map = map.clone();
    let mut seen = HashSet::new();

    return map
        .iter()
        .flat_map(|line| line.iter().collect::<Vec<_>>())
        .map(|c| c.clone())
        .filter(|x| *x != '.' && seen.insert(x.clone()))
        .collect::<Vec<char>>();
}

pub fn draw_antinodes(map: &AntennaMap, antinodes: &Vec<Location>) {
    let mut map = map.clone();

    for (x, y) in antinodes {
        map[*y as usize][*x as usize] = '#';
    }

    let map = map
        .iter()
        .map(|line| {
            return String::from_iter(line);
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", map);
}
