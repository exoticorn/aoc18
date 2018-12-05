use crate::prelude::*;
use std::collections::HashSet;

fn polymer_from_str(s: &str) -> Vec<u8> {
    s.trim().chars().map(|c| c as u8).collect()
}

fn reduce_polymer(p: &mut Vec<u8>) {
    let mut i = 0;
    let mut j = 0;
    while i < p.len() {
        if j == 0 || p[j - 1] ^ 32 != p[i] {
            p[j] = p[i];
            j += 1;
        } else {
            j -= 1;
        }
        i += 1;
    }
    p.truncate(j);
}

pub fn run(data: &AocData) -> AocResult {
    let polymer = polymer_from_str(&data.to_string()?);

    let mut poly_part1 = polymer.clone();
    reduce_polymer(&mut poly_part1);

    let unit_types: HashSet<u8> = polymer.iter().map(|v| v | 32).collect();
    let best_length = unit_types
        .into_iter()
        .map(|unit_type| {
            let mut poly: Vec<u8> = polymer
                .iter()
                .cloned()
                .filter(|&c| c | 32 != unit_type)
                .collect();
            reduce_polymer(&mut poly);
            poly.len()
        })
        .min()
        .ok_or_else(|| format_err!("No unit types found in polymer"))?;

    answers(poly_part1.len(), best_length)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let mut poly = polymer_from_str("dabAcCaCBAcCcaDA");
        reduce_polymer(&mut poly);
        assert_eq!(poly.len(), 10);

        let mut poly = polymer_from_str("ADdabAcCaCBAcCcaDAada");
        reduce_polymer(&mut poly);
        assert_eq!(poly.len(), 7);
    }
}
