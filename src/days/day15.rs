use crate::array2d::Array2d;
use crate::prelude::*;

pub fn run(data: &AocData) -> AocResult {
    let map = load_map(data)?;
    let mut simulation = Simulation::new(map);
    simulation.run();
    answer(simulation.outcome())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Fighter { elf: bool, hp: u8 },
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Empty
    }
}

fn load_map(data: &AocData) -> Result<Array2d<Tile>> {
    let lines: Vec<String> = data.lines()?.collect();
    if lines.is_empty() {
        bail!("Empty map");
    }
    let mut map = Array2d::new(lines[0].len(), lines.len());

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'G' => Tile::Fighter {
                    elf: false,
                    hp: 200,
                },
                'E' => Tile::Fighter { elf: true, hp: 200 },
                _ => bail!("Unexpected char in map: '{}'", c),
            };
            map.put(x, y, tile);
        }
    }

    Ok(map)
}

struct Simulation {
    map: Array2d<Tile>,
    moved: Array2d<bool>,
    targets: Array2d<bool>,
    distances: Array2d<u8>,
    num_rounds: usize,
}

impl Simulation {
    fn new(map: Array2d<Tile>) -> Simulation {
        let width = map.width();
        let height = map.height();
        Simulation {
            map,
            moved: Array2d::new(width, height),
            targets: Array2d::new(width, height),
            distances: Array2d::new(width, height),
            num_rounds: 0,
        }
    }

    fn run(&mut self) {
        while !self.is_over() {
            self.run_turn();
        }
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        println!(
            "after turn {}\n{}",
            self.num_rounds,
            self.map.to_string(|tile| match *tile {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Fighter { elf, .. } => {
                    if elf {
                        'E'
                    } else {
                        'G'
                    }
                }
            })
        );
    }

    fn outcome(&self) -> usize {
        let (a, b) = self.hp_sum();
        (a + b) * self.num_rounds
    }

    fn hp_sum(&self) -> (usize, usize) {
        let mut hp_elves = 0;
        let mut hp_goblins = 0;
        for (x, y) in self.map.iter_xy() {
            match self.map[(x, y)] {
                Tile::Fighter { hp, elf: true } => hp_elves += hp as usize,
                Tile::Fighter { hp, elf: false } => hp_goblins += hp as usize,
                _ => (),
            }
        }
        (hp_elves, hp_goblins)
    }

    fn is_over(&self) -> bool {
        let (hp_elves, hp_goblins) = self.hp_sum();
        hp_elves == 0 || hp_goblins == 0
    }

    fn run_turn(&mut self) {
        self.moved.fill(false);
        let mut out_of_targets = false;
        for (x, y) in self.map.iter_xy() {
            if self.moved[(x, y)] {
                continue;
            }
            let fighter = self.map[(x, y)];
            match fighter {
                Tile::Fighter { elf, .. } => {
                    if !self.mark_targets(!elf) {
                        out_of_targets = true;
                    };
                    let (x, y) = if let Some((tx, ty)) = self.fill_distance_to_targets(x, y) {
                        let (mx, my) = self.backtrace_to_fighter(tx, ty);
                        self.map[(x, y)] = Tile::Empty;
                        self.map[(mx, my)] = fighter;
                        self.moved[(mx, my)] = true;
                        (mx, my)
                    } else {
                        (x, y)
                    };
                    self.attack(x, y, !elf);
                }
                _ => (),
            }
        }
        if !out_of_targets {
            self.num_rounds += 1;
        }
    }

    fn mark_targets(&mut self, elves: bool) -> bool {
        self.targets.fill(false);
        let mut found_targets = false;
        for (x, y) in self.map.iter_xy() {
            match self.map[(x, y)] {
                Tile::Fighter { elf, .. } if elf == elves => {
                    found_targets = true;
                    for (x, y) in neighbors(x, y) {
                        self.targets.put(x, y, true);
                    }
                }
                _ => (),
            }
        }
        found_targets
    }

    fn fill_distance_to_targets(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        if self.targets.get(x, y) {
            return None;
        }
        self.distances.fill(0);
        for (x, y) in neighbors(x, y) {
            if self.map.get(x, y) == Tile::Empty {
                self.distances.put(x, y, 1);
                if self.targets.get(x, y) {
                    return Some((x, y));
                }
            }
        }
        for d in 1.. {
            let mut found = false;
            let mut reached_target = false;
            for (x, y) in self.map.iter_xy() {
                if self.distances.get(x, y) == d {
                    found = true;
                    for (x, y) in neighbors(x, y) {
                        if self.map.get(x, y) == Tile::Empty && self.distances.get(x, y) == 0 {
                            self.distances.put(x, y, d + 1);
                            if self.targets.get(x, y) {
                                reached_target = true;
                            }
                        }
                    }
                }
            }
            if !found {
                return None;
            }
            if reached_target {
                for (x, y) in self.map.iter_xy() {
                    if self.distances.get(x, y) == d + 1 && self.targets.get(x, y) {
                        return Some((x, y));
                    }
                }
            }
        }
        unreachable!()
    }

    fn backtrace_to_fighter(&mut self, x: usize, y: usize) -> (usize, usize) {
        self.targets.fill(false);
        self.targets.put(x, y, true);
        for dist in (1..=self.distances.get(x, y)).rev() {
            for (x, y) in self.map.iter_xy() {
                if self.distances.get(x, y) == dist && self.targets.get(x, y) {
                    if dist == 1 {
                        return (x, y);
                    }
                    for (x, y) in neighbors(x, y) {
                        self.targets.put(x, y, true);
                    }
                }
            }
        }
        unreachable!()
    }

    fn attack(&mut self, x: usize, y: usize, elves: bool) {
        if let Some(min_hp) = neighbors(x, y)
            .filter_map(|(x, y)| match self.map.get(x, y) {
                Tile::Fighter { hp, elf } if elf == elves => Some(hp),
                _ => None,
            })
            .min()
        {
            for (x, y) in neighbors(x, y) {
                match self.map.get(x, y) {
                    Tile::Fighter { hp, elf } if hp == min_hp && elf == elves => {
                        let new_hp = hp.saturating_sub(3);
                        self.map.put(
                            x,
                            y,
                            if new_hp > 0 {
                                Tile::Fighter { hp: new_hp, elf }
                            } else {
                                Tile::Empty
                            },
                        );
                        break;
                    }
                    _ => (),
                }
            }
        }
    }
}

const OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    OFFSETS
        .iter()
        .map(move |&(ox, oy)| (((x as isize) + ox) as usize, ((y as isize) + oy) as usize))
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_testcase(s: &'static str, outcome: usize) {
        let data = AocData::from_str(s);
        let mut simulation = Simulation::new(load_map(&data).unwrap());
        simulation.run();
        assert_eq!(simulation.outcome(), outcome);
    }

    #[test]
    fn case0() {
        run_testcase(
            "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
        ",
            27730,
        );
    }

    #[test]
    fn case1() {
        run_testcase(
            "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
        ",
            36334,
        );
    }
}
