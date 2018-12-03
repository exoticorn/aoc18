use crate::prelude::*;

mod day1;
mod day2;
mod day3;

pub const DAYS: [fn(&AocData) -> AocResult; 3] =
    [self::day1::run, self::day2::run, self::day3::run];
