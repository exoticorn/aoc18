use crate::array2d::Array2d;
use crate::prelude::*;
use std::cmp;
use std::collections::BTreeSet;
use std::mem;

pub fn run(data: &AocData) -> AocResult {
    let (map, (ox, oy)) = create_map(&data.to_string()?)?;
    answers(
        longest_path(&map, ox, oy),
        count_distant_rooms(&map, ox, oy, 1000),
    )
}

type Group = Vec<Vec<Part>>;

enum Part {
    String(String),
    Group(Group),
}

fn parse_re(re: &str) -> Result<Group> {
    let mut chars = re.chars();
    if chars.next() != Some('^') {
        bail!("Expected ^ at start of regex");
    }
    let group = parse_group(&mut chars)?;
    if let Some(c) = chars.next() {
        bail!("Expected end of regex, found '{}'", c);
    }
    Ok(group)
}

fn parse_group(chars: &mut dyn Iterator<Item = char>) -> Result<Group> {
    let mut str_part = String::new();
    let mut branch = vec![];
    let mut group = vec![];

    while let Some(c) = chars.next() {
        match c {
            'N' | 'E' | 'S' | 'W' => str_part.push(c),
            ')' | '$' => {
                branch.push(Part::String(str_part));
                group.push(branch);
                return Ok(group);
            }
            '|' => {
                branch.push(Part::String(str_part));
                str_part = String::new();
                group.push(branch);
                branch = vec![];
            }
            '(' => {
                branch.push(Part::String(str_part));
                str_part = String::new();
                branch.push(Part::Group(parse_group(chars)?));
            }
            _ => bail!("Unexpected char '{}'", c),
        }
    }

    bail!("Unexpected end of input")
}

fn walk_path<F>(group: &Group, mut f: F)
where
    F: FnMut(isize, isize, u8),
{
    fn walk<F>(
        group: &Group,
        positions: BTreeSet<(isize, isize)>,
        f: &mut F,
    ) -> BTreeSet<(isize, isize)>
    where
        F: FnMut(isize, isize, u8),
    {
        let mut end_positions = BTreeSet::new();

        for branch in group {
            let mut positions = positions.clone();
            for part in branch {
                match *part {
                    Part::String(ref s) => {
                        positions = positions
                            .into_iter()
                            .map(|(mut x, mut y)| {
                                for c in s.chars() {
                                    match c {
                                        'N' => {
                                            y -= 1;
                                            f(x, y, 0);
                                        }
                                        'E' => {
                                            x += 1;
                                            f(x, y, 1);
                                        }
                                        'S' => {
                                            y += 1;
                                            f(x, y, 2);
                                        }
                                        'W' => {
                                            x -= 1;
                                            f(x, y, 3);
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                (x, y)
                            })
                            .collect();
                    }
                    Part::Group(ref g) => {
                        positions = walk(g, positions, f);
                    }
                }
            }
            end_positions.extend(positions.into_iter());
        }

        end_positions
    }
    let mut positions = BTreeSet::new();
    positions.insert((0isize, 0isize));
    walk(group, positions, &mut f);
}

type DoorMap = Array2d<u8>;

fn create_map(s: &str) -> Result<(DoorMap, (usize, usize))> {
    let group = parse_re(s)?;

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);

    walk_path(&group, |x, y, _| {
        min_x = cmp::min(min_x, x);
        min_y = cmp::min(min_y, y);
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);
    });

    let mut map = Array2d::new((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize);

    walk_path(&group, |x, y, d| {
        let (x, y) = ((x - min_x) as usize, (y - min_y) as usize);
        match d {
            0 => map[(x, y + 1)] |= 1,
            1 => map[(x, y)] |= 2,
            2 => map[(x, y)] |= 1,
            3 => map[(x + 1, y)] |= 2,
            _ => unreachable!(),
        }
    });

    Ok((map, (-min_x as usize, -min_y as usize)))
}

#[allow(dead_code)]
fn print_map(s: &str) -> Result<()> {
    let (map, _) = create_map(s)?;

    for y in 0..map.height() {
        for x in 0..map.width() {
            print!("#{}", if map[(x, y)] & 1 == 0 { '#' } else { '-' });
        }
        println!("#");
        for x in 0..map.width() {
            print!("{}.", if map[(x, y)] & 2 == 0 { '#' } else { '|' });
        }
        println!("#");
    }
    for _ in 0..map.width() {
        print!("##");
    }
    println!("#");
    Ok(())
}

fn longest_path(map: &DoorMap, start_x: usize, start_y: usize) -> usize {
    let mut length = 0usize;
    pathfind(map, start_x, start_y, |d| length = d);
    length
}

fn count_distant_rooms(
    map: &DoorMap,
    start_x: usize,
    start_y: usize,
    min_distance: usize,
) -> usize {
    let mut count = 0;
    pathfind(map, start_x, start_y, |d| {
        if d >= min_distance {
            count += 1
        }
    });
    count
}

fn pathfind<F>(map: &DoorMap, start_x: usize, start_y: usize, mut f: F)
where
    F: FnMut(usize),
{
    let mut visited: Array2d<bool> = Array2d::new(map.width(), map.height());

    let mut next_positions = BTreeSet::new();
    next_positions.insert((start_x, start_y));

    for i in 0usize.. {
        for (x, y) in mem::replace(&mut next_positions, BTreeSet::new()).into_iter() {
            visited[(x, y)] = true;
            f(i);
            if map[(x, y)] & 1 != 0 && !visited.get(x, y.wrapping_sub(1)) {
                next_positions.insert((x, y - 1));
            }
            if map[(x, y)] & 2 != 0 && !visited.get(x.wrapping_sub(1), y) {
                next_positions.insert((x - 1, y));
            }
            if map.get(x, y + 1) & 1 != 0 && !visited.get(x, y + 1) {
                next_positions.insert((x, y + 1));
            }
            if map.get(x + 1, y) & 2 != 0 && !visited.get(x + 1, y) {
                next_positions.insert((x + 1, y));
            }
        }
        if next_positions.is_empty() {
            break;
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let (map, (x, y)) = create_map("^ENWWW(NEEE|SSE(EE|N))$").unwrap();
    assert_eq!(longest_path(&map, x, y), 10);

    let (map, (x, y)) = create_map("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").unwrap();
    assert_eq!(longest_path(&map, x, y), 18);
}
