type Input = Vec<Vec<i64>>;

pub fn generator(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn next_sequence(input: &[i64]) -> Vec<i64> {
    input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn samesies(input: &[i64]) -> bool {
    input.iter().all(|&val| val == input[0])
}

fn extrapolate(input: &[i64]) -> i64 {
    let next = next_sequence(input);
    if samesies(&next) {
        return input.last().unwrap() + next[0];
    }

    let extrapolated = extrapolate(&next);
    input.last().unwrap() + extrapolated
}

pub fn part_1(input: &Input) -> i64 {
    input.iter().map(|seq| extrapolate(seq)).sum()
}

fn lextrapolate(input: &[i64]) -> i64 {
    let next = next_sequence(input);
    if samesies(&next) {
        return input.first().unwrap() - next[0];
    }

    let extrapolated = lextrapolate(&next);
    input.first().unwrap() - extrapolated
}

pub fn part_2(input: &Input) -> i64 {
    input.iter().map(|seq| lextrapolate(seq)).sum()
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_generator() {
        assert_debug_snapshot!(generator(&INPUT));
    }

    #[test]
    fn test_next_sequence() {
        assert_eq!(
            next_sequence(&vec![0, 3, 6, 9, 12, 15]),
            vec![3, 3, 3, 3, 3]
        );
        assert_eq!(next_sequence(&vec![3, 3, 3, 3, 3]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 114);
    }

    #[test]
    fn test_lextrapolate() {
        assert_eq!(lextrapolate(&vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(lextrapolate(&vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(lextrapolate(&vec![10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_part_2() {
        let input = generator(&INPUT);
        assert_eq!(part_2(&input), 2);
    }
}
