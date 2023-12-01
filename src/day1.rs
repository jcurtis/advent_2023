use itertools::Itertools;
use phf::phf_map;

pub fn part_1(input: &str) -> u32 {
    input.lines().map(line).sum()
}

fn line(input: &str) -> u32 {
    let input = input.trim();
    if input.is_empty() {
        return 0;
    }

    let first = input.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = input.chars().rfind(|c| c.is_ascii_digit()).unwrap();
    format!("{}{}", first, last).parse().unwrap()
}

static NUMS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static MAP: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn line_2(input: &str) -> u32 {
    let input = input.trim();
    if input.is_empty() {
        return 0;
    }

    // (index, value)
    let first_digit = input.chars().enumerate().find(|(_, c)| c.is_ascii_digit());
    let last_digit = input.chars().enumerate().collect_vec();
    let last_digit = last_digit.iter().rfind(|(_, c)| c.is_ascii_digit());

    // (index, value)
    let spelled_indexes = NUMS
        .iter()
        .flat_map(|&num| input.match_indices(num).map(|(index, _)| (index, MAP[num])))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .collect_vec();
    let first_spelled = spelled_indexes.first();
    let last_spelled = spelled_indexes.last();

    let first = if first_digit.is_some() && first_spelled.is_some() {
        let first_digit = first_digit.unwrap();
        let first_spelled = first_spelled.unwrap();
        if first_digit.0 < first_spelled.0 {
            first_digit.1.to_digit(10).unwrap()
        } else {
            first_spelled.1
        }
    } else if let Some(first_digit) = first_digit {
        first_digit.1.to_digit(10).unwrap()
    } else {
        first_spelled.unwrap().1
    };

    let last = if last_digit.is_some() && last_spelled.is_some() {
        let last_digit = last_digit.unwrap();
        let last_spelled = last_spelled.unwrap();
        if last_digit.0 > last_spelled.0 {
            last_digit.1.to_digit(10).unwrap()
        } else {
            last_spelled.1
        }
    } else if last_digit.is_some() {
        last_digit.unwrap().1.to_digit(10).unwrap()
    } else {
        last_spelled.unwrap().1
    };

    // dbg!(
    //     &input,
    //     &spelled_indexes,
    //     &first_digit,
    //     &first_spelled,
    //     &first,
    //     &last_digit,
    //     &last_spelled,
    //     &last
    // );

    format!("{}{}", first, last).parse().unwrap()
}

pub fn part_2(input: &str) -> u32 {
    input.trim().lines().map(line_2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        assert_eq!(line("1abc2"), 12);
        assert_eq!(line("pqr3stu8vwx"), 38);
        assert_eq!(line("a1b2c3d4e5f"), 15);
        assert_eq!(line("treb7uchet"), 77);
    }

    #[test]
    fn test_part_1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n ";
        assert_eq!(part_1(&input), 142);
    }

    #[test]
    fn test_line_2() {
        assert_eq!(line_2("two1nine"), 29);
        assert_eq!(line_2("eightwothree"), 83);
        assert_eq!(line_2("abcone2threexyz"), 13);
        assert_eq!(line_2("xtwone3four"), 24);
        assert_eq!(line_2("4nineeightseven2"), 42);
        assert_eq!(line_2("zoneight234"), 14);
        assert_eq!(line_2("7pqrstsixteen"), 76);
        assert_eq!(line_2("oneeighttwo34dcjck5eightjznpzhxdlc"), 18);
    }

    #[test]
    fn test_part_2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n ";
        assert_eq!(part_2(&input), 281);
    }
}
