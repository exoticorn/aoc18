use crate::array2d::Array2d;
use crate::prelude::*;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;

pub fn run(data: &AocData) -> AocResult {
    let mut field = parse_data(data)?;
    let mut field2 = field.clone();
    let resources10 = resource_value_after(&mut field, 10);
    let resources1000000000 = resource_value_after(&mut field2, 1000000000);
    answers(resources10, resources1000000000)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    None,
    Open,
    Wooded,
    Lumberyard,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::None
    }
}

type Field = Array2d<Tile>;

fn parse_data(data: &AocData) -> Result<Field> {
    let lines: Vec<String> = data.lines()?.collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut map = Array2d::new(width, lines.len());
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.put(
                x,
                y,
                match c {
                    '.' => Tile::Open,
                    '|' => Tile::Wooded,
                    '#' => Tile::Lumberyard,
                    _ => bail!("Unexpected tile '{}'", c),
                },
            );
        }
    }
    Ok(map)
}

fn resource_value_after(field: &mut Field, minutes: usize) -> usize {
    let mut dest = Field::new(field.width(), field.height());
    let mut cycle: HashMap<u64, usize> = HashMap::new();
    for minute in 1..=minutes {
        step(field, &mut dest);
        mem::swap(field, &mut dest);
        let mut hasher = DefaultHasher::new();
        field.hash(&mut hasher);
        let hash = hasher.finish();
        match cycle.entry(hash) {
            Entry::Vacant(vac) => {
                vac.insert(minute);
            }
            Entry::Occupied(mut occ) => {
                if (minutes - minute) % (minute - *occ.get()) == 0 {
                    return resource_value(&field);
                } else {
                    occ.insert(minute);
                }
            }
        }
    }
    resource_value(&field)
}

fn step(src: &Field, dest: &mut Field) {
    for (x, y) in src.iter_xy() {
        let mut num_wooded = 0;
        let mut num_lumberyard = 0;
        for yo in 0..3 {
            for xo in 0..3 {
                if xo != 1 || yo != 1 {
                    match src.get((x + xo).wrapping_sub(1), (y + yo).wrapping_sub(1)) {
                        Tile::Wooded => num_wooded += 1,
                        Tile::Lumberyard => num_lumberyard += 1,
                        _ => (),
                    }
                }
            }
            dest.put(
                x,
                y,
                match src.get(x, y) {
                    Tile::Open if num_wooded >= 3 => Tile::Wooded,
                    Tile::Wooded if num_lumberyard >= 3 => Tile::Lumberyard,
                    Tile::Lumberyard if num_wooded == 0 || num_lumberyard == 0 => Tile::Open,
                    o => o,
                },
            )
        }
    }
}

fn resource_value(field: &Field) -> usize {
    let mut num_wooded = 0;
    let mut num_lumberyard = 0;
    for (x, y) in field.iter_xy() {
        match field[(x, y)] {
            Tile::Wooded => num_wooded += 1,
            Tile::Lumberyard => num_lumberyard += 1,
            _ => (),
        }
    }
    num_wooded * num_lumberyard
}

#[allow(dead_code)]
fn print_field(field: &Field) {
    println!(
        "{}",
        field.to_string(|t| match t {
            Tile::Open => '.',
            Tile::Wooded => '|',
            Tile::Lumberyard => '#',
            Tile::None => '?',
        })
    );
}

#[cfg(test)]
#[test]
fn test() {
    let data = AocData::from_str(
        "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
    ",
    );
    let mut field = parse_data(&data).unwrap();
    assert_eq!(resource_value_after(&mut field, 10), 1147);
}
