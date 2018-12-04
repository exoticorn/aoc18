use crate::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;

pub const DAYS: [fn(&AocData) -> AocResult; 4] = [
    self::day1::run,
    self::day2::run,
    self::day3::run,
    self::day4::run,
];
