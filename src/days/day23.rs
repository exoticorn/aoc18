use crate::prelude::*;

pub fn run(data: &AocData) -> AocResult {
    let bots = parse_input(data)?;
    answer(num_in_range_of_strongest(&bots))
}

struct Bot {
    x: i32,
    y: i32,
    z: i32,
    r: u32,
}

impl Bot {
    fn dist(&self, x: i32, y: i32, z: i32) -> u32 {
        (x - self.x).abs() as u32 + (y - self.y).abs() as u32 + (z - self.z).abs() as u32
    }

    fn in_range(&self, x: i32, y: i32, z: i32) -> bool {
        self.dist(x, y, z) <= self.r
    }
}

fn parse_input(data: &AocData) -> Result<Vec<Bot>> {
    let mut bots = Vec::new();
    let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    for line in data.lines()? {
        if let Some(caps) = re.captures(&line) {
            bots.push(Bot {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap(),
                r: caps[4].parse().unwrap(),
            });
        } else {
            bail!("Failed to parse line: {}", line);
        }
    }
    if bots.is_empty() {
        bail!("No bots found in input");
    }
    Ok(bots)
}

fn num_in_range_of_strongest(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.r).unwrap();
    bots.iter()
        .filter(|b| strongest.in_range(b.x, b.y, b.z))
        .count()
}
