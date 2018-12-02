use crate::prelude::*;
use std::collections::HashSet;

pub fn run(data: &AocData) -> AocResult {
    let changes: Vec<i32> = data.values()?.collect();
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
