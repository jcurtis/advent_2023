use itertools::Itertools;
use std::collections::HashMap;

type Input = HashMap<(usize, usize), char>;

pub fn generator(input: &str) -> Input {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(col, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(row, c)| match c {
                    '.' => None,
                    c => Some(((row, col), c)),
                })
                .collect_vec()
        })
        .collect()
}

static UP: [char; 3] = ['|', '7', 'F'];
static RIGHT: [char; 3] = ['-', 'J', '7'];
static DOWN: [char; 3] = ['|', 'L', 'J'];
static LEFT: [char; 3] = ['-', 'L', 'F'];

fn find_dir(
    input: &Input,
    &(x, y): &(usize, usize),
    from: &(usize, usize),
    pipe: char,
) -> (usize, usize) {
    // up
    if DOWN.contains(&pipe) && y > 0 {
        let pos = (x, y - 1);
        if &pos != from && input.contains_key(&pos) {
            return pos;
        }
    }

    // right
    let pos = (x + 1, y);
    if LEFT.contains(&pipe) && &pos != from && input.contains_key(&pos) {
        return pos;
    }

    // down
    let pos = (x, y + 1);
    if UP.contains(&pipe) && &pos != from && input.contains_key(&pos) {
        return pos;
    }

    // left
    if RIGHT.contains(&pipe) && x > 0 {
        let pos = (x - 1, y);
        if &pos != from && input.contains_key(&pos) {
            return pos;
        }
    }

    unreachable!();
}

fn find_start_dir(input: &Input, &(x, y): &(usize, usize)) -> ((usize, usize), (usize, usize)) {
    let mut res = vec![];
    // println!("find_start_dir from {:?}", (x, y));

    // up
    if y > 0 {
        let pos = (x, y - 1);
        if let Some(pipe) = input.get(&pos) {
            if UP.contains(pipe) {
                // println!("up found {} at {:?}", &pipe, &pos);
                res.push(pos);
            }
        }
    }

    // right
    let pos = (x + 1, y);
    if let Some(pipe) = input.get(&pos) {
        if RIGHT.contains(pipe) {
            // println!("right found {} at {:?}", &pipe, &pos);
            res.push(pos);
        }
    }

    // down
    let pos = (x, y + 1);
    if let Some(pipe) = input.get(&pos) {
        if DOWN.contains(pipe) {
            // println!("down found {} at {:?}", &pipe, &pos);
            res.push(pos);
        }
    }

    // left
    if x > 0 {
        let pos = (x - 1, y);
        if let Some(pipe) = input.get(&pos) {
            if LEFT.contains(pipe) {
                // println!("left found {} at {:?}", &pipe, &pos);
                res.push(pos);
            }
        }
    }

    if res.len() != 2 {
        dbg!(&res);
        unreachable!("Invalid start directions");
    }

    (res[0], res[1])
}

pub fn part_1(input: &Input) -> usize {
    let start = find_start(&input);
    let (mut dir_1, mut dir_2) = find_start_dir(&input, start);
    let mut prev_pos_1 = start.clone();
    let mut prev_pos_2 = start.clone();
    let mut count = 1;

    while dir_1 != dir_2 {
        if count > 10000 {
            unreachable!("cut early");
        }

        let pipe_1 = input[&dir_1];
        let pipe_2 = input[&dir_2];

        let new_dir_1 = find_dir(&input, &dir_1, &prev_pos_1, pipe_1);
        let new_dir_2 = find_dir(&input, &dir_2, &prev_pos_2, pipe_2);

        prev_pos_1 = dir_1;
        prev_pos_2 = dir_2;

        dir_1 = new_dir_1;
        dir_2 = new_dir_2;

        count += 1;
    }

    count
}

fn find_start(input: &Input) -> &(usize, usize) {
    input
        .iter()
        .find_map(|(pos, c)| match c {
            'S' => Some(pos),
            _ => None,
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    static INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    static INPUT_COMPLEX: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_generator() {
        assert_debug_snapshot!(generator(&INPUT).iter().sorted().collect_vec());
        assert_debug_snapshot!(generator(&INPUT_COMPLEX).iter().sorted().collect_vec());
    }

    #[test]
    fn test_find_start() {
        let input = generator(&INPUT);
        assert_eq!(find_start(&input), &(1, 1));

        let input = generator(&INPUT_COMPLEX);
        assert_eq!(find_start(&input), &(0, 2));
    }

    #[test]
    fn test_find_start_dir() {
        let input = generator(&INPUT);
        let start = find_start(&input);
        assert_eq!(find_start_dir(&input, start), ((2, 1), (1, 2)));
    }

    #[test]
    fn test_part_1() {
        let input = generator(&INPUT);
        assert_eq!(part_1(&input), 4);

        let input = generator(&INPUT_COMPLEX);
        assert_eq!(part_1(&input), 8);
    }
}
