use crate::prelude::*;

mod day1;
mod day2;

pub const DAYS: [fn(&AocData) -> AocResult; 2] = [self::day1::run, self::day2::run];
