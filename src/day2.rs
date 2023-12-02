use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

lazy_static! {
    static ref GAME_ID_RE: Regex = Regex::new(r"Game (\d+)").unwrap();
    static ref DICE_RE: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
}

fn parse_game(input: &str) -> Game {
    let id = GAME_ID_RE
        .captures(input)
        .map(|cap| cap.get(1).unwrap().as_str().parse().unwrap())
        .unwrap();

    let sets = input
        .split(';')
        .map(|set| {
            DICE_RE
                .captures_iter(set)
                .fold(Set::default(), |mut set, cap| {
                    let num = cap[1].parse().unwrap();
                    match &cap[2] {
                        "red" => {
                            set.red = num;
                        }
                        "green" => {
                            set.green = num;
                        }
                        "blue" => {
                            set.blue = num;
                        }
                        _ => unreachable!(),
                    };
                    set
                })
        })
        .collect_vec();

    Game { id, sets }
}

pub fn generator(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect_vec()
}

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

pub fn part_1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter_map(|game| {
            if game
                .sets
                .iter()
                .all(|set| set.red <= MAX_RED && set.green <= MAX_GREEN && set.blue <= MAX_BLUE)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn game_power(game: &Game) -> u32 {
    let max = game.sets.iter().fold(Set::default(), |acc, set| Set {
        red: acc.red.max(set.red),
        green: acc.green.max(set.green),
        blue: acc.blue.max(set.blue),
    });
    max.red * max.green * max.blue
}

pub fn part_2(input: &[Game]) -> u32 {
    input.iter().map(game_power).sum()
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn test_parse_game() {
        assert_debug_snapshot!(parse_game(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        ))
    }

    lazy_static! {
        static ref INPUT: String = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
    }

    #[test]
    fn test_generator() {
        assert_debug_snapshot!(generator(&INPUT));
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 8);
    }

    #[test]
    fn test_game_power() {
        let input = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game_power(&input), 48);

        let input = parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(game_power(&input), 12);
    }

    #[test]
    fn test_part_2() {
        let input = generator(&INPUT);
        assert_eq!(part_2(&input), 2286);
    }
}
