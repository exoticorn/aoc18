use crate::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub const DAYS: &'static [fn(&AocData) -> AocResult] = &[
    self::day1::run,
    self::day2::run,
    self::day3::run,
    self::day4::run,
    self::day5::run,
];
