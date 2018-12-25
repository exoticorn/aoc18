use crate::array2d::Array2d;
use crate::prelude::*;
use std::cmp;
use std::i32;

pub fn run(data: &AocData) -> AocResult {
    let bots = parse_input(data)?;
    let closest_max = intersect_bots(&bots);
    answers(num_in_range_of_strongest(&bots), closest_max)
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

type IntersectingBots = Array2d<bool>;

fn intersect_bots(bots: &[Bot]) -> i32 {
    let mut intersections = Array2d::new(bots.len(), bots.len());
    for (i, bot_a) in bots.iter().enumerate() {
        for (j, bot_b) in bots.iter().enumerate() {
            if bot_a.dist(bot_b.x, bot_b.y, bot_b.z) < bot_a.r + bot_b.r {
                intersections[(i, j)] = true;
            }
        }
    }

    fn add_to_set(
        bots: &[Bot],
        intersections: &IntersectingBots,
        mut set: Vec<bool>,
        start: usize,
        max_size: &mut usize,
        closest_max: &mut i32,
    ) {
        let mut size = set.iter().filter(|&&b| b).count();
        let mut any = false;
        for i in start..set.len() {
            if size < *max_size {
                return;
            }
            if !set[i] {
                continue;
            }
            any = true;
            let mut new_set = set.clone();
            for j in (i + 1)..set.len() {
                if !intersections[(i, j)] {
                    new_set[j] = false;
                }
            }
            add_to_set(bots, intersections, new_set, i + 1, max_size, closest_max);
            set[i] = false;
            size -= 1;
        }

        if !any {
            if size >= *max_size {
                let mut b = Box4d::all();
                for (i, bot) in bots.iter().enumerate() {
                    if set[i] {
                        b = b.intersect(&bot_to_box(bot)).unwrap();
                    }
                }
                fn dist(min: i32, max: i32) -> i32 {
                    if min < 0 && max > 0 {
                        0
                    } else {
                        cmp::min(min.abs(), max.abs())
                    }
                }
                *closest_max = cmp::max(
                    dist(b.a_min, b.a_max),
                    cmp::max(
                        dist(b.b_min, b.b_max),
                        cmp::max(dist(b.c_min, b.c_max), dist(b.d_min, b.d_max)),
                    ),
                );
                *max_size = size;
            }
        }
    }

    let mut max_size = 0;
    let mut closest_max = i32::MAX;
    for i in 0..bots.len() {
        let mut set = vec![false; bots.len()];
        for j in i..bots.len() {
            set[j] = intersections[(i, j)];
        }
        add_to_set(
            bots,
            &intersections,
            set,
            i + 1,
            &mut max_size,
            &mut closest_max,
        );
    }

    closest_max
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
    fn all() -> Box4d {
        Box4d {
            a_min: i32::MIN,
            a_max: i32::MAX,
            b_min: i32::MIN,
            b_max: i32::MAX,
            c_min: i32::MIN,
            c_max: i32::MAX,
            d_min: i32::MIN,
            d_max: i32::MAX,
        }
    }

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
