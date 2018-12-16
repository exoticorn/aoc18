use crate::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const DAYS: &'static [fn(&AocData) -> AocResult] = &[
    self::day1::run,
    self::day2::run,
    self::day3::run,
    self::day4::run,
    self::day5::run,
    self::day6::run,
    self::day7::run,
    self::day8::run,
    self::day9::run,
    self::day10::run,
    self::day11::run,
    self::day12::run,
    self::day13::run,
    self::day14::run,
    self::day15::run,
    self::day16::run,
];
