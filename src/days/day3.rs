use crate::prelude::*;
use std::cmp;

struct Claim {
    id: usize,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub fn run(data: &AocData) -> AocResult {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claims = Vec::new();
    for line in data.lines()? {
        let captures = re
            .captures(&line)
            .ok_or_else(|| format_err!("Failed to parse line '{}'", line))?;
        claims.push(Claim {
            id: captures[1].parse().unwrap(),
            x: captures[2].parse().unwrap(),
            y: captures[3].parse().unwrap(),
            width: captures[4].parse().unwrap(),
            height: captures[5].parse().unwrap(),
        });
    }

    let mut field: Vec<Vec<u8>> = Vec::new();
    for claim in &claims {
        field.resize(
            cmp::max(field.len(), (claim.y + claim.height) as usize),
            Vec::new(),
        );
        for y in claim.y..(claim.y + claim.height) {
            let row = &mut field[y as usize];
            row.resize(cmp::max(row.len(), (claim.x + claim.width) as usize), 0);
            for x in claim.x..(claim.x + claim.width) {
                row[x as usize] = row[x as usize].saturating_add(1);
            }
        }
    }

    let overlapping_squares: usize = field
        .iter()
        .map(|row| row.iter().filter(|&&c| c > 1).count())
        .sum();

    let nonoverlaping_claim = claims
        .into_iter()
        .find(|claim| {
            (claim.y..(claim.y + claim.height)).all(|y| {
                let row = &field[y as usize];
                (claim.x..(claim.x + claim.width)).all(|x| row[x as usize] == 1)
            })
        })
        .ok_or_else(|| format_err!("Failed to find non-overlapping claim"))?;

    answers(overlapping_squares, nonoverlaping_claim.id)
}
