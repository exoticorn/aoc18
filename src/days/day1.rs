use crate::prelude::*;

pub fn run(data: &AocData) -> AocResult {
    for line in data.lines()? {
        println!("{}", line);
    }
    answer(23)
}
