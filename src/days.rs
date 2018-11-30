use crate::prelude::*;

mod day1;

pub const DAYS: [fn(&AocData) -> AocResult; 1] = [self::day1::run];
