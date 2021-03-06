use crate::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
    self::day17::run,
    self::day18::run,
    self::day19::run,
    self::day20::run,
    self::day21::run,
    self::day22::run,
    self::day23::run,
    self::day24::run,
    self::day25::run,
];
