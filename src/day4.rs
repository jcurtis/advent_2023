use std::collections::HashSet;

use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    input.trim().lines().map(calc_card).sum()
}

fn parse_nums(input: &str) -> HashSet<u32> {
    input
        .trim()
        .split(' ')
        .filter_map(|num| {
            if !num.is_empty() {
                Some(num.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .collect()
}

fn card_wins(line: &str) -> u32 {
    let split = line.split(&[':', '|']).collect_vec();
    let winning = parse_nums(split[1]);
    let mine = parse_nums(split[2]);
    mine.iter().filter(|num| winning.contains(num)).count() as u32
}

fn calc_card(line: &str) -> u32 {
    let count_wins = card_wins(line);
    if count_wins == 0 {
        0
    } else {
        2_u32.pow((count_wins).saturating_sub(1))
    }
}

pub fn part_2(input: &str) -> u32 {
    let cards = input.trim().lines().map(card_wins).collect_vec();
    let len = cards.len();
    let mut wins: Vec<u32> = vec![1; len];
    for index in 0..len {
        let card = cards[index] as usize;
        for inc_index in 1..=card.min(len) {
            wins[index + inc_index] += wins[index];
        }
    }
    wins.iter().sum()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_check_line() {
        let lines = INPUT.lines().collect_vec();
        assert_eq!(calc_card(lines[0]), 8);
        assert_eq!(calc_card(lines[1]), 2);
        assert_eq!(calc_card(lines[2]), 2);
        assert_eq!(calc_card(lines[3]), 1);
        assert_eq!(calc_card(lines[4]), 0);
        assert_eq!(calc_card(lines[5]), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&INPUT), 30);
    }
}
