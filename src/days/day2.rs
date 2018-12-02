use crate::prelude::*;
use std::collections::HashMap;

fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut m = HashMap::new();
    for c in s.chars() {
        *m.entry(c).or_insert(0usize) += 1;
    }
    m
}

fn distance(a: &str, b: &str) -> usize {
    assert!(a.len() == b.len());
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

fn common_chars(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

fn find_close(ids: &[String]) -> Option<String> {
    for i in 0..(ids.len() - 1) {
        for b in &ids[(i + 1)..] {
            if distance(&ids[i], b) == 1 {
                return Some(common_chars(&ids[i], b));
            }
        }
    }
    None
}

pub fn run(data: &AocData) -> AocResult {
    let ids: Vec<_> = data.lines()?.collect();
    let mut count_twice = 0usize;
    let mut count_thrice = 0usize;
    for line in &ids {
        let counts = char_counts(line);
        if counts.values().any(|&c| c == 2) {
            count_twice += 1;
        }
        if counts.values().any(|&c| c == 3) {
            count_thrice += 1;
        }
    }
    let second = find_close(&ids).ok_or_else(|| format_err!("No close ids found"))?;
    answers(count_twice * count_thrice, second)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn counts() {
        assert_eq!(char_counts("abbcde")[&'b'], 2);
    }

    #[test]
    fn dist() {
        assert_eq!(distance("abcde", "axcye"), 2);
        assert_eq!(distance("fghij", "fguij"), 1);
    }

    #[test]
    fn rem_com() {
        assert_eq!(common_chars("fghij", "fguij"), "fgij");
    }
}
