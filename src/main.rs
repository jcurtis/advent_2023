mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

aoc_main::main! {
    year 2023;
    day1 => part_1, part_2;
    day2: generator => part_1, part_2;
    day3 => part_1, part_2;
    day4 => part_1, part_2;
    day5: generator => part_1, part_2;
    day6: generator => part_1, part_2;
    day7: generator => part_1, part_2;
}
