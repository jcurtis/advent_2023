use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

lazy_static! {
    static ref NUM_REG: Regex = Regex::new(r"\d+").unwrap();
    static ref SYMBOLS_REG: Regex = Regex::new(r"[^\d\.\n]").unwrap();
    static ref GEARS_REG: Regex = Regex::new(r"\*").unwrap();
}

pub fn part_1(input: &str) -> u32 {
    let lines = input.lines().collect_vec();
    input
        .lines()
        .enumerate()
        .flat_map(parse_parts)
        .filter_map(|part| {
            if part.row > 0 {
                let len = lines[part.row - 1].len() - 1;
                let section = lines[part.row - 1]
                    [(part.pos.start.max(1) - 1)..(part.pos.end + 1).min(len)]
                    .to_string();
                if SYMBOLS_REG.is_match(&section) {
                    return Some(part.id);
                }
            }

            {
                let len = lines[part.row].len() - 1;
                let section = lines[part.row]
                    [(part.pos.start.max(1) - 1)..(part.pos.end + 1).min(len)]
                    .to_string();
                if SYMBOLS_REG.is_match(&section) {
                    return Some(part.id);
                }
            }

            if part.row != lines.len() - 1 {
                let len = lines[part.row + 1].len() - 1;
                let section = lines[part.row + 1]
                    [(part.pos.start.max(1) - 1)..(part.pos.end + 1).min(len)]
                    .to_string();
                if SYMBOLS_REG.is_match(&section) {
                    return Some(part.id);
                }
            }

            None
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Part {
    id: u32,
    row: usize,
    pos: Range<usize>,
}

fn parse_parts((row, line): (usize, &str)) -> Vec<Part> {
    NUM_REG
        .captures_iter(line)
        .map(|cap| {
            let part_match = cap.get(0).unwrap();
            Part {
                id: part_match.as_str().parse().unwrap(),
                pos: part_match.start()..part_match.end(),
                row,
            }
        })
        .collect()
}

pub fn part_2(input: &str) -> u32 {
    let parts_by_row: HashMap<usize, Vec<Part>> = input
        .lines()
        .enumerate()
        .flat_map(parse_parts)
        .group_by(|part| part.row)
        .into_iter()
        .fold(HashMap::new(), |mut acc, (row, parts)| {
            acc.insert(row, parts.collect_vec());
            acc
        });

    let gears = input
        .lines()
        .enumerate()
        .flat_map(parse_gears)
        .collect_vec();

    gears
        .iter()
        .filter_map(|gear| {
            let mut found_parts = vec![];
            let row = gear.row;

            // Row before
            if let Some(section) = parts_by_row.get(&(row - 1)) {
                section.iter().for_each(|part| {
                    if part.pos.start <= gear.col + 1 && gear.col <= part.pos.end {
                        found_parts.push(part.clone());
                    }
                });
            }

            // Same row
            if let Some(section) = parts_by_row.get(&row) {
                // println!("cur {:?}", &section);
                section.iter().for_each(|part| {
                    if part.pos.start <= gear.col + 1 && gear.col <= part.pos.end {
                        found_parts.push(part.clone());
                    }
                });
            }

            // Row after
            if let Some(section) = parts_by_row.get(&(row + 1)) {
                section.iter().for_each(|part| {
                    if part.pos.start <= gear.col + 1 && gear.col <= part.pos.end {
                        found_parts.push(part.clone());
                    }
                });
            }

            // dbg!(&gear, &found_parts);
            if found_parts.len() == 2 {
                Some(found_parts.iter().map(|part| part.id).product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug)]
struct Gear {
    row: usize,
    col: usize,
}

fn parse_gears((row, line): (usize, &str)) -> Vec<Gear> {
    GEARS_REG
        .captures_iter(line)
        .map(|cap| Gear {
            row,
            col: cap.get(0).unwrap().start(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_parts((0, "467..114..")),
            vec![
                Part {
                    id: 467,
                    pos: 0..3,
                    row: 0
                },
                Part {
                    id: 114,
                    pos: 5..8,
                    row: 0
                }
            ]
        )
    }

    static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&INPUT), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&INPUT), 467835);
    }
}
