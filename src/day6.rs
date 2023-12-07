use std::iter::zip;

use itertools::Itertools;

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

pub fn generator(input: &str) -> Vec<Race> {
    let lines = input.lines().collect_vec();
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();
    zip(&times, &distances)
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn calc_race(hold_time: u64, total_time: u64) -> u64 {
    let move_time = total_time - hold_time;
    move_time * hold_time
}

pub fn part_1(input: &[Race]) -> u64 {
    input
        .iter()
        .map(|race| {
            (1..race.time)
                .filter(|&hold_time| calc_race(hold_time, race.time) > race.distance)
                .count() as u64
        })
        .product()
}

pub fn part_2(input: &[Race]) -> u64 {
    let input = input.iter().fold(
        Race {
            time: 0,
            distance: 0,
        },
        |acc, race| Race {
            time: format!("{}{}", acc.time, race.time).parse().unwrap(),
            distance: format!("{}{}", acc.distance, race.distance)
                .parse()
                .unwrap(),
        },
    );
    let input = vec![input];
    part_1(&input)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_generator() {
        assert_debug_snapshot!(generator(&INPUT), @r###"
        [
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ]
        "###);
    }

    #[test]
    fn test_calc_race() {
        assert_eq!(calc_race(0, 7), 0);
        assert_eq!(calc_race(1, 7), 6);
        assert_eq!(calc_race(2, 7), 10);
        assert_eq!(calc_race(3, 7), 12);
        assert_eq!(calc_race(4, 7), 12);
        assert_eq!(calc_race(5, 7), 10);
        assert_eq!(calc_race(6, 7), 6);
        assert_eq!(calc_race(7, 7), 0);
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 288);
    }

    #[test]
    fn test_part_2() {
        let input = generator(&INPUT);
        assert_eq!(part_2(&input), 71503);
    }
}
