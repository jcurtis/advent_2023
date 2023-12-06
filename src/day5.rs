use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

type Seed = u64;
type Mapping = String;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Map {
    to: String,
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

type Maps = HashMap<Mapping, Vec<Map>>;

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<Seed>,
    maps: Maps,
}

lazy_static! {
    static ref MAPPING_RE: Regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
}

pub fn generator(input: &str) -> Almanac {
    let seeds = input.lines().next().unwrap();
    let seeds = seeds
        .split(' ')
        .skip(1)
        .map(|num| num.parse::<Seed>().unwrap())
        .collect_vec();

    let maps: Maps = input
        .split("\n\n")
        .skip(1)
        .map(|section| {
            let mapping = MAPPING_RE.captures(section).unwrap();
            (
                mapping[1].to_string(),
                section
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let nums = line
                            .split(' ')
                            .map(|num| num.parse::<u64>().unwrap())
                            .collect_vec();
                        Map {
                            to: mapping[2].to_string(),
                            destination_start: nums[0],
                            source_start: nums[1],
                            range_length: nums[2],
                        }
                    })
                    .collect_vec(),
            )
        })
        .collect();

    Almanac { seeds, maps }
}

fn seed_location(seed: Seed, almanac: &Almanac) -> u64 {
    let location = "location".to_string();
    let mut cur_map = "seed".to_string();
    let mut cur_index = seed;
    let mut count = 0;

    // println!("0 - soil {cur_index}");
    while cur_map.ne(&location) && count < 100 {
        let found_maps = almanac.maps.get(&cur_map).unwrap();
        let found_map = found_maps.iter().find(|map| {
            cur_index >= map.source_start && cur_index < map.source_start + map.range_length
        });
        cur_index = if let Some(map) = found_map {
            let offset = cur_index - map.source_start;
            map.destination_start + offset
        } else {
            cur_index
        };

        cur_map = found_maps[0].to.clone();
        count += 1;

        // println!("{count} - {} {cur_index}", &cur_map);
    }

    cur_index
}

pub fn part_1(almanac: &Almanac) -> Seed {
    almanac
        .seeds
        .iter()
        .map(|seed| seed_location(*seed, almanac))
        .min()
        .unwrap()
}

pub fn part_2(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .chunks(2)
        .map(|range| {
            let range = range[0]..(range[0] + range[1]);
            println!("checking range {:?}", &range);
            range
                .into_par_iter()
                .map(|seed| seed_location(seed, almanac))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    lazy_static! {
        static ref ALMANAC: Almanac = generator(&INPUT);
    }

    #[test]
    fn test_generator() {
        assert_eq!(ALMANAC.seeds, vec![79, 14, 55, 13]);
        let sorted_map = ALMANAC.maps.iter().sorted();
        assert_debug_snapshot!(&sorted_map);
    }

    #[test]
    fn test_seed_location() {
        assert_eq!(seed_location(79, &ALMANAC), 82);
        assert_eq!(seed_location(14, &ALMANAC), 43);
        assert_eq!(seed_location(55, &ALMANAC), 86);
        assert_eq!(seed_location(13, &ALMANAC), 35);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&ALMANAC), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&ALMANAC), 46);
    }

    static INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
}
