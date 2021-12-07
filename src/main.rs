#![feature(slice_group_by)]
#![feature(iter_advance_by)]

mod day1;
mod day2;
mod day3;
mod day4;

aoc_main::main! {
    year 2021;
    day1 : generate => part1, part2;
    day2 : generate => part1, part2;
    day3 : generate => part1, part2;
    day4 : generate => part1, part2;
}
