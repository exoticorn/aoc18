use crate::array2d::Array2d;
use crate::prelude::*;
use std::cmp;
use std::ops::RangeInclusive;
use std::usize;

pub fn run(data: &AocData) -> AocResult {
    let (mut map, offsets) = parse_input(data)?;
    put_water(&mut map, offsets.spring, 0);
    let (reachable, resting) = count_water(&map, &offsets);
    answers(reachable + resting, resting)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Tile {
    Sand,
    Clay,
    Resting,
    Reachable,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Sand
    }
}

struct MapOffsets {
    spring: usize,
    min_y: usize,
    max_y: usize,
}

fn parse_input(data: &AocData) -> Result<(Array2d<Tile>, MapOffsets)> {
    let mut rects = vec![];
    let re = Regex::new(r"(x|y)=(\d+)(\.\.(\d+))?").unwrap();

    for line in data.lines()? {
        let mut x: Option<RangeInclusive<usize>> = None;
        let mut y: Option<RangeInclusive<usize>> = None;
        for caps in re.captures_iter(&line) {
            let from: usize = caps[2].parse().unwrap();
            let range = if let Some(cap) = caps.get(4) {
                from..=(cap.as_str().parse().unwrap())
            } else {
                from..=from
            };
            if &caps[1] == "x" {
                x = Some(range);
            } else {
                y = Some(range);
            }
        }
        if let (Some(x), Some(y)) = (x, y) {
            rects.push((x, y));
        } else {
            bail!("Failed to parse: {}", line);
        }
    }

    if rects.is_empty() {
        bail!("No input");
    }

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (500, 500, usize::MAX, 0);
    for rect in &rects {
        min_x = cmp::min(min_x, rect.0.start() - 1);
        max_x = cmp::max(max_x, rect.0.end() + 1);
        min_y = cmp::min(min_y, *rect.1.start());
        max_y = cmp::max(max_y, *rect.1.end());
    }

    let mut map = Array2d::new(max_x - min_x + 1, max_y + 1);

    for (xr, yr) in rects.into_iter() {
        for y in yr {
            for x in xr.clone() {
                map[(x - min_x, y)] = Tile::Clay;
            }
        }
    }

    Ok((
        map,
        MapOffsets {
            spring: 500 - min_x,
            min_y,
            max_y,
        },
    ))
}

fn put_water(map: &mut Array2d<Tile>, x: usize, y: usize) -> bool {
    if y >= map.height() {
        return false;
    }

    match map.get(x, y + 1) {
        Tile::Sand => {
            if !put_water(map, x, y + 1) {
                map.put(x, y, Tile::Reachable);
                return false;
            }
        }
        Tile::Reachable => {
            map.put(x, y, Tile::Reachable);
            return false;
        }
        _ => (),
    }

    fn walk<F>(map: &mut Array2d<Tile>, mut x: usize, y: usize, f: F) -> (usize, bool)
    where
        F: Fn(usize) -> usize,
    {
        loop {
            if map.get(f(x), y) == Tile::Clay {
                return (x, true);
            }
            x = f(x);
            match map.get(x, y + 1) {
                Tile::Sand => {
                    if !put_water(map, x, y + 1) {
                        return (x, false);
                    }
                }
                Tile::Reachable => return (x, false),
                _ => (),
            }
        }
    }

    let (left_x, blocked_left) = walk(map, x, y, |x| x - 1);
    let (right_x, blocked_right) = walk(map, x, y, |x| x + 1);

    let tile = if blocked_left && blocked_right {
        Tile::Resting
    } else {
        Tile::Reachable
    };
    for x in left_x..=right_x {
        map.put(x, y, tile);
    }

    tile == Tile::Resting
}

fn count_water(map: &Array2d<Tile>, offsets: &MapOffsets) -> (usize, usize) {
    let mut count_reachable = 0;
    let mut count_resting = 0;
    for y in offsets.min_y..=offsets.max_y {
        for x in 0..map.width() {
            match map[(x, y)] {
                Tile::Resting => count_resting += 1,
                Tile::Reachable => count_reachable += 1,
                _ => (),
            }
        }
    }
    (count_reachable, count_resting)
}

#[allow(dead_code)]
fn print_map(map: &Array2d<Tile>) {
    println!(
        "{}",
        map.to_string(|&t| match t {
            Tile::Sand => '.',
            Tile::Clay => '#',
            Tile::Resting => '~',
            Tile::Reachable => '|',
        })
    );
}

#[cfg(test)]
#[test]
fn test() {
    let data = AocData::from_str(
        "
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
    ",
    );

    let (mut map, offsets) = parse_input(&data).unwrap();

    put_water(&mut map, offsets.spring, 0);

    print_map(&map);

    assert_eq!(count_water(&map, &offsets), 57);
}
