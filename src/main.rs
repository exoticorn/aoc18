#[macro_use]
extern crate quicli;

use quicli::prelude::*;

mod array2d;
mod days;
mod prelude;

use self::days::DAYS;
use self::prelude::AocData;

#[derive(Debug, StructOpt)]
struct Cli {
    day: Option<usize>,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn run_day(day: usize) -> Result<()> {
    if day < 1 || day > DAYS.len() {
        bail!("Day {} out of range", day);
    }
    let data = AocData::new(day);
    let answers = DAYS[day - 1](&data)?;
    data.ok()?;
    println!("Day {}: {}", day, answers);
    Ok(())
}

main!(
    |args: Cli, log_level: verbosity| if let Some(day) = args.day {
        run_day(day)?;
    } else {
        for day in 1..=DAYS.len() {
            run_day(day)?;
        }
    }
);
