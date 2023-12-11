use itertools::Itertools;
use std::{collections::HashSet, iter::zip};

type Input = HashSet<(usize, usize)>;

pub fn generator(input: &str) -> Input {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(column_index, c)| match c {
                    '#' => Some((column_index, line_index)),
                    _ => None,
                })
                .collect_vec()
        })
        .collect()
}

fn expand_by(input: &Input, by: usize) -> Input {
    let by = by - 1;
    let (left, right): (Vec<usize>, Vec<usize>) = input.iter().cloned().unzip();
    let bounds_x = *left.iter().max().unwrap();
    let bounds_y = *right.iter().max().unwrap();

    let left_range = 0..bounds_x;
    let left_to_increment = left_range.filter(|x| !left.contains(x)).collect_vec();
    let left = left
        .iter()
        .map(|&x| {
            let increment_times = left_to_increment
                .iter()
                .filter(|&to_increment| &x > to_increment)
                .count();
            x + (increment_times * by)
        })
        .collect_vec();

    let right_range = 0..bounds_y;
    let right_to_increment = right_range.filter(|x| !right.contains(x)).collect_vec();
    let right = right
        .iter()
        .map(|&y| {
            let increment_times = right_to_increment
                .iter()
                .filter(|&to_increment| &y > to_increment)
                .count();
            y + (increment_times * by)
        })
        .collect_vec();

    zip(left, right).collect()
}

fn dist(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn solve(input: &Input, expand_by_size: usize) -> usize {
    let expanded = expand_by(&input, expand_by_size);
    expanded
        .iter()
        .combinations(2)
        .map(|comb| dist(comb[0], comb[1]))
        .sum()
}

pub fn part_1(input: &Input) -> usize {
    solve(input, 2)
}

pub fn part_2(input: &Input) -> usize {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    static INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_generator() {
        assert_debug_snapshot!(generator(&INPUT).iter().sorted().collect_vec());
    }

    #[test]
    fn test_expand() {
        let input = generator(&INPUT);
        let expected = generator(
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......",
        );

        assert_eq!(expand_by(&input, 2), expected);
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 374);
    }

    #[test]
    fn test_solve() {
        let input = generator(&INPUT);
        assert_eq!(solve(&input, 10), 1030);
        assert_eq!(solve(&input, 100), 8410);
    }
}
