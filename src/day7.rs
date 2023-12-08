use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, Copy)]
struct Hand([u32; 5]);

#[derive(Debug, Eq)]
pub struct Play {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum HandRank {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl Hand {
    fn hand_rank(&self) -> HandRank {
        let Hand(hand) = self;
        let groups = hand.iter().counts();

        // handle jokers (1)
        let hand = if groups.contains_key(&1) {
            // println!("contains joker {:?}", &hand);
            let max = hand.iter().max().unwrap();
            let new_hand: [u32; 5] = hand
                .iter()
                .map(|val| match val {
                    1 => *max,
                    val => *val,
                })
                .collect_vec()
                .try_into()
                .unwrap();
            // dbg!(&hand, &max, &new_hand);
            // println!("new hand {:?}", &res);
            new_hand
        } else {
            *hand
        };

        let groups = hand.iter().counts();
        let key_count = groups.keys().count();

        match key_count {
            5 => HandRank::High,
            4 => HandRank::Pair,
            3 => {
                if groups.values().any(|&hits| hits == 3) {
                    HandRank::Three
                } else {
                    HandRank::TwoPair
                }
            }
            2 => match groups.values().next().unwrap() {
                4 | 1 => HandRank::Four,
                2 | 3 => HandRank::Full,
                _ => {
                    // dbg!(&hand, &groups);
                    unreachable!();
                }
            },
            1 => HandRank::Five,
            _ => unreachable!(),
        }
    }

    // remap joker to 1
    fn remap_joker(&self) -> Self {
        Hand(
            self.0
                .iter()
                .map(|&val| match val {
                    11 => 1,
                    val => val,
                })
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let Hand(self_hand) = self;
        let Hand(other_hand) = other;
        match self.hand_rank().cmp(&other.hand_rank()) {
            Ordering::Equal => {
                // println!("equal rank? {:?} == {:?}", &self_hand, &other_hand);
                if let Some(ordering) =
                    self_hand
                        .iter()
                        .zip(other_hand.iter())
                        .find_map(|(&a, &b)| match a.cmp(&b) {
                            Ordering::Equal => None,
                            ordering => Some(ordering),
                        })
                {
                    // println!("resolved to {:?}", ordering);
                    ordering
                } else {
                    // println!("equal? {:?} == {:?}", &self_hand, &other_hand);
                    Ordering::Equal
                }
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

fn parse_hand(hand: &str) -> Hand {
    Hand(
        hand.chars()
            .map(map_char_to_value)
            .collect_vec()
            .try_into()
            .unwrap(),
    )
}

pub fn generator(input: &str) -> Vec<Play> {
    input
        .trim()
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect_vec();
            Play {
                hand: parse_hand(split[0]),
                bid: split[1].parse().unwrap(),
            }
        })
        .collect()
}

fn map_char_to_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        card => card.to_digit(10).unwrap(),
    }
}

pub fn part_1(input: &[Play]) -> u32 {
    input
        .iter()
        .sorted()
        .enumerate()
        .map(|(index, play)| (index as u32 + 1) * play.bid)
        .sum()
}

// too low 247281796
pub fn part_2(input: &[Play]) -> u32 {
    input
        .iter()
        .map(|play| Play {
            hand: play.hand.remap_joker(),
            bid: play.bid,
        })
        .sorted()
        .enumerate()
        .map(|(index, play)| (index as u32 + 1) * play.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_generator() {
        assert_debug_snapshot!(generator(&INPUT));
    }

    #[test]
    fn test_hand_rank() {
        assert_eq!(HandRank::Five.cmp(&HandRank::Four), Ordering::Greater);
        assert_eq!(Hand([13, 13, 13, 13, 13]).hand_rank(), HandRank::Five);
        assert_eq!(Hand([12, 13, 13, 13, 13]).hand_rank(), HandRank::Four);
        assert_eq!(Hand([12, 12, 13, 13, 13]).hand_rank(), HandRank::Full);
        assert_eq!(Hand([11, 12, 13, 13, 13]).hand_rank(), HandRank::Three);
        assert_eq!(Hand([11, 12, 12, 13, 13]).hand_rank(), HandRank::TwoPair);
        assert_eq!(Hand([11, 12, 10, 13, 13]).hand_rank(), HandRank::Pair);
        assert_eq!(Hand([11, 12, 10, 9, 13]).hand_rank(), HandRank::High);

        // With active jokers
        assert_eq!(
            Hand([11, 11, 11, 11, 11]).remap_joker().hand_rank(),
            HandRank::Five
        );
        assert_eq!(
            Hand([12, 11, 11, 11, 11]).remap_joker().hand_rank(),
            HandRank::Five
        );
        assert_eq!(
            Hand([12, 12, 11, 11, 11]).remap_joker().hand_rank(),
            HandRank::Five
        );
        assert_eq!(
            Hand([2, 12, 11, 11, 11]).remap_joker().hand_rank(),
            HandRank::Four
        );
        assert_eq!(
            Hand([2, 12, 12, 11, 11]).remap_joker().hand_rank(),
            HandRank::Four
        );
        assert_eq!(
            Hand([2, 12, 10, 11, 11]).remap_joker().hand_rank(),
            HandRank::Three
        );
        assert_eq!(
            Hand([2, 12, 10, 9, 11]).remap_joker().hand_rank(),
            HandRank::Pair
        );
    }

    #[test]
    fn test_hand_ordering() {
        let plays = generator(&INPUT);
        assert_debug_snapshot!(plays.iter().sorted().collect_vec());

        let a = Hand([12, 12, 12, 12, 2]).remap_joker();
        assert_eq!(a.hand_rank(), HandRank::Four);
        let b = Hand([11, 13, 13, 13, 2]).remap_joker();
        assert_eq!(b.hand_rank(), HandRank::Four);
        assert_eq!(
            Hand([13, 13, 13, 11, 2]).remap_joker().hand_rank(),
            HandRank::Four
        );

        assert_eq!(
            Hand([12, 12, 12, 12, 2])
                .remap_joker()
                .cmp(&Hand([11, 13, 13, 13, 2]).remap_joker()),
            Ordering::Greater
        );

        assert_eq!(
            Hand([12, 12, 12, 12, 2])
                .remap_joker()
                .cmp(&Hand([13, 13, 13, 11, 2]).remap_joker()),
            Ordering::Less
        );

        assert_eq!(
            Hand([11, 12, 12, 12, 2])
                .remap_joker()
                .cmp(&Hand([13, 13, 13, 13, 2]).remap_joker()),
            Ordering::Less
        );

        assert_eq!(
            Hand([11, 11, 11, 11, 11])
                .remap_joker()
                .cmp(&Hand([2, 2, 2, 2, 2]).remap_joker()),
            Ordering::Less
        );
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 6440);
    }

    #[test]
    fn test_remap_joker() {
        assert_eq!(Hand([0, 0, 0, 0, 0]).remap_joker(), Hand([0, 0, 0, 0, 0]));
        assert_eq!(Hand([11, 0, 0, 0, 0]).remap_joker(), Hand([1, 0, 0, 0, 0]));
        assert_eq!(Hand([11, 0, 1, 2, 3]).remap_joker(), Hand([1, 0, 1, 2, 3]));
        assert_eq!(
            Hand([11, 11, 11, 11, 11]).remap_joker(),
            Hand([1, 1, 1, 1, 1])
        );
    }

    #[test]
    fn test_part_2() {
        let input = generator(&INPUT);
        assert_eq!(part_2(&input), 5905);
    }
}
