use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{collections::HashMap, mem};

type Key = String;
type Nodes = HashMap<Key, (Key, Key)>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Nav {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Input {
    nav: Vec<Nav>,
    nodes: Nodes,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
}

pub fn generator(input: &str) -> Input {
    let nav = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|nav| match nav {
            'R' => Nav::Right,
            'L' => Nav::Left,
            _ => unreachable!(),
        })
        .collect();

    let nodes = input
        .trim()
        .lines()
        .skip(2)
        .map(|node| {
            RE.captures(node)
                .map(|cap| (cap[1].to_string(), (cap[2].to_string(), cap[3].to_string())))
                .unwrap()
        })
        .collect();

    Input { nav, nodes }
}

pub fn part_1(input: &Input) -> usize {
    traverse_shortest(input, "AAA")
}

fn traverse_shortest(input: &Input, from: &str) -> usize {
    let mut nav = input.nav.iter().cycle();
    let mut pos = from;
    let mut found_z = false;
    let mut count = 0;

    while !found_z && count < 100000 {
        count += 1;
        let next_nav = nav.next().unwrap();
        pos = match next_nav {
            Nav::Left => &input.nodes[pos].0,
            Nav::Right => &input.nodes[pos].1,
        };
        found_z = pos.chars().nth(2).unwrap() == 'Z';
    }

    count
}

pub fn part_2(input: &Input) -> u128 {
    let shortest: Vec<u128> = input
        .nodes
        .keys()
        .filter(|&key| matches!(key.chars().nth(2).unwrap(), 'A'))
        .collect_vec()
        .par_iter()
        .map(|pos| traverse_shortest(input, pos) as u128)
        .collect();
    lcmm(&shortest)
}

fn gcd(a: u128, b: u128) -> u128 {
    if a == b {
        return a;
    }

    let mut a = a;
    let mut b = b;

    if b > a {
        mem::swap(&mut a, &mut b);
    }

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}

fn lcmm(nums: &[u128]) -> u128 {
    nums.iter().fold(1, |a, &b| lcm(a, b))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use itertools::Itertools;

    use super::*;

    static INPUT: &str = " RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_generator() {
        let input = generator(&INPUT);
        assert_eq!(input.nav, vec![Nav::Right, Nav::Left]);
        assert_debug_snapshot!(input.nodes.iter().sorted());
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 2);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let input = generator(&input);
        assert_eq!(part_1(&input), 6);
    }

    #[test]
    fn test_part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let input = generator(&input);
        assert_eq!(part_2(&input), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 3), 6);

        assert_eq!(lcmm(&vec![2, 3]), 6);
        assert_eq!(lcmm(&vec![2, 3, 6]), 6);
        assert_eq!(lcmm(&vec![100, 23, 98]), 112700);
    }
}
