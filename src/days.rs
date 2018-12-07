use crate::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub const DAYS: &'static [fn(&AocData) -> AocResult] = &[
    self::day1::run,
    self::day2::run,
    self::day3::run,
    self::day4::run,
    self::day5::run,
    self::day6::run,
    self::day7::run,
];
