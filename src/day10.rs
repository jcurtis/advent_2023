use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
    let start = find_start(input);
    let (mut dir_1, mut dir_2) = find_start_dir(input, start);
    let mut prev_pos_1 = *start;
    let mut prev_pos_2 = *start;
    let mut count = 1;

    while dir_1 != dir_2 {
        if count > 10000 {
            unreachable!("cut early");
        }

        let pipe_1 = input[&dir_1];
        let pipe_2 = input[&dir_2];

        let new_dir_1 = find_dir(input, &dir_1, &prev_pos_1, pipe_1);
        let new_dir_2 = find_dir(input, &dir_2, &prev_pos_2, pipe_2);

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

fn build_path(input: &Input) -> HashSet<(usize, usize)> {
    let start = find_start(input);

    let mut prev_pos = *start;
    let (mut pos, _) = find_start_dir(input, start);

    let mut path = HashSet::new();
    path.insert(prev_pos);
    path.insert(pos);
    let mut count = 0;

    while &pos != start {
        count += 1;
        if count > 100000 {
            unreachable!("cut early");
        }

        let new_pos = find_dir(input, &pos, &prev_pos, input[&pos]);
        prev_pos = pos;
        pos = new_pos;
        path.insert(pos);
    }

    path
}

fn ray_cast(row: &[char]) -> usize {
    let mut count = 0;
    let mut iter = row.iter();

    // println!("for row {:?}", &row);
    while let Some(pipe) = iter.next() {
        // println!("take {pipe}");
        match pipe {
            '|' => count += 1,
            'L' => {
                for &next_pipe in iter.by_ref() {
                    if next_pipe == '7' {
                        count += 1;
                        break;
                    } else if next_pipe == 'J' {
                        break;
                    }
                }
            }
            'F' => {
                for &next_pipe in iter.by_ref() {
                    if next_pipe == 'J' {
                        count += 1;
                        break;
                    } else if next_pipe == '7' {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    count
}

// too high 465
pub fn part_2(input: &Input) -> usize {
    let path: HashSet<(usize, usize)> = build_path(input);

    // Find bounds
    let (left, right): (Vec<usize>, Vec<usize>) = path.iter().cloned().unzip();
    let bounds_x = *left.iter().max().unwrap();
    let bounds_y = *right.iter().max().unwrap();

    // Find points that need to be tested
    let mut count = 0;
    for x in 0..bounds_x {
        for y in 0..bounds_y {
            if !path.contains(&(x, y)) {
                // println!("testing ({x}, {y})");
                // empty point (x, y) - is in inside?
                let row = path
                    .iter()
                    .filter(|&tile| tile.0 < x && tile.1 == y)
                    .sorted_by(|&a, &b| a.0.cmp(&b.0))
                    .map(|&tile| input[&tile])
                    .collect_vec();
                if !row.is_empty() {
                    let cross_count = ray_cast(&row);
                    println!(
                        "testing ({x}, {y}) {:?} - counted {cross_count}",
                        &row.iter().cloned().collect::<String>()
                    );

                    if cross_count % 2 == 1 {
                        count += 1;
                    }
                }
            }
        }
    }
    count
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

    #[test]
    fn test_part_2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        println!("\n{input}");
        let input = generator(&input);
        assert_eq!(part_2(&input), 4);

        let input = "...........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        println!("\n{input}");
        let input = generator(&input);
        assert_eq!(part_2(&input), 4);

        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        println!("\n{input}");
        let input = generator(&input);
        assert_eq!(part_2(&input), 8);

        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        println!("\n{input}");
        let input = generator(&input);
        assert_eq!(part_2(&input), 10);
    }
}
