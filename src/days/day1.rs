use crate::prelude::*;
use std::collections::HashSet;

pub fn run(data: &AocData) -> AocResult {
    let changes = data
        .lines()?
        .map(|l| l.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    let first: i32 = changes.iter().sum();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut freq = 0i32;
    let second = 'outer: loop {
        for &change in &changes {
            if !seen.insert(freq) {
                break 'outer freq;
            }
            freq += change;
        }
    };
    answers(first, second)
}
