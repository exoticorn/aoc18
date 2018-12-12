use crate::prelude::*;

use std::collections::{hash_map::Entry, HashMap};
use std::iter;

#[derive(Debug, Clone)]
struct Pots {
    first: isize,
    pots: Vec<bool>,
}

type Rules = u32;

impl Pots {
    fn step(&self, rules: Rules) -> Pots {
        let mut w = 0;
        let pots_iter = self
            .pots
            .iter()
            .cloned()
            .chain(iter::repeat(false).take(4))
            .map(|f| {
                w = (w >> 1) | ((f as u32) << 4);
                rules & (1 << w) != 0
            });
        let mut first = self.first - 3;
        let mut pots: Vec<_> = pots_iter
            .skip_while(|&f| {
                first += 1;
                !f
            })
            .collect();
        while pots.len() > 1 && !pots[pots.len() - 1] {
            pots.truncate(pots.len() - 1);
        }
        Pots { first, pots }
    }

    fn iter_flowers<'a>(&'a self) -> impl Iterator<Item = isize> + 'a {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, &f)| f)
            .map(move |(i, _)| i as isize + self.first)
    }
}

fn parse_input(lines: &mut Iterator<Item = String>) -> Result<(Pots, Rules)> {
    let initial_state = lines
        .next()
        .ok_or_else(|| format_err!("initial state missing"))?;
    let pots = if let Some(cap) = Regex::new(r"^initial state: ([#.]+)$")
        .unwrap()
        .captures(&initial_state)
    {
        Pots {
            first: 0,
            pots: cap[1].chars().map(|c| c == '#').collect(),
        }
    } else {
        bail!("Failed to parse initial state: {}", initial_state);
    };

    if !lines.next().iter().any(|s| s == "") {
        bail!("Expected empty line after initial state");
    }

    let re = Regex::new(r"^([#.]{5}) => ([#.])$").unwrap();
    let mut rules = 0u32;
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            if &cap[2] == "#" {
                let i = cap[1]
                    .chars()
                    .enumerate()
                    .fold(0, |a, (i, c)| if c == '#' { a | (1 << i) } else { a });
                rules |= 1 << i;
            }
        } else {
            bail!("Failed to parse rule: {}", line);
        }
    }

    Ok((pots, rules))
}

fn flower_sum(pots: &Pots, rules: Rules, generations: usize) -> isize {
    let mut pots = pots.clone();
    let mut cycle_map: HashMap<Vec<bool>, (usize, isize)> = HashMap::new();
    for generation in 1.. {
        pots = pots.step(rules);
        let sum = pots.iter_flowers().sum();
        if generation == generations {
            return sum;
        }
        match cycle_map.entry(pots.pots.clone()) {
            Entry::Occupied(mut occ) => {
                let (prev_generation, prev_sum) = occ.insert((generation, sum));
                let cycle_length = generation - prev_generation;
                if (generations - generation) % cycle_length == 0 {
                    let cycle_sum_offset = sum - prev_sum;
                    return sum
                        + cycle_sum_offset * ((generations - generation) / cycle_length) as isize;
                }
            }
            Entry::Vacant(vac) => {
                vac.insert((generation, sum));
            }
        }
    }
    unreachable!();
}

pub fn run(data: &AocData) -> AocResult {
    let (pots, rules) = parse_input(&mut data.lines()?)?;

    let flowers_20 = flower_sum(&pots, rules, 20);
    let flowers_50000000000 = flower_sum(&pots, rules, 50000000000);

    answers(flowers_20, flowers_50000000000)
}
