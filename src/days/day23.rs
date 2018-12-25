use crate::prelude::*;
use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::i32;

pub fn run(data: &AocData) -> AocResult {
    let bots = parse_input(data)?;
    intersect_bots(&bots);
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

fn intersect_bots(bots: &[Bot]) {
    let mut new_boxes: Vec<(Box4d, usize)> = vec![];
    let mut boxes: HashMap<Box4d, usize> = HashMap::new();

    for (index, bot) in bots.iter().enumerate() {
        let bot_box = bot_to_box(bot);
        for (box_, &count) in boxes.iter() {
            if let Some(intersection) = box_.intersect(&bot_box) {
                new_boxes.push((intersection, count + 1));
            }
        }
        new_boxes.push((bot_box, 1));
        print!("Adding bot {}, {} new boxes...", index + 1, new_boxes.len());
        for (box_, count) in new_boxes.drain(..) {
            match boxes.entry(box_) {
                Entry::Occupied(mut occ) => {
                    let c = occ.get_mut();
                    *c = cmp::max(*c, count);
                }
                Entry::Vacant(vac) => {
                    vac.insert(count);
                }
            }
        }
        println!("{} boxes total", boxes.len());
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Box4d {
    a_min: i32,
    a_max: i32,
    b_min: i32,
    b_max: i32,
    c_min: i32,
    c_max: i32,
    d_min: i32,
    d_max: i32,
}

impl Box4d {
    fn intersect(&self, o: &Box4d) -> Option<Box4d> {
        let new_box = Box4d {
            a_min: cmp::max(self.a_min, o.a_min),
            a_max: cmp::min(self.a_max, o.a_max),
            b_min: cmp::max(self.b_min, o.b_min),
            b_max: cmp::min(self.b_max, o.b_max),
            c_min: cmp::max(self.c_min, o.c_min),
            c_max: cmp::min(self.c_max, o.c_max),
            d_min: cmp::max(self.d_min, o.d_min),
            d_max: cmp::min(self.d_max, o.d_max),
        };
        if new_box.a_max > new_box.a_min
            && new_box.b_max > new_box.b_min
            && new_box.c_max > new_box.c_min
            && new_box.d_max > new_box.d_min
        {
            Some(new_box)
        } else {
            None
        }
    }
}

fn bot_to_box(bot: &Bot) -> Box4d {
    let a = bot.x + bot.y + bot.z;
    let b = bot.x + bot.y - bot.z;
    let c = bot.x - bot.y + bot.z;
    let d = bot.x - bot.y - bot.z;
    let r = bot.r as i32;
    Box4d {
        a_min: a - r,
        a_max: a + r + 1,
        b_min: b - r,
        b_max: b + r + 1,
        c_min: c - r,
        c_max: c + r + 1,
        d_min: d - r,
        d_max: d + r + 1,
    }
}
