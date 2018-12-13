use crate::prelude::*;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Track {
    None,
    Straight,
    Slash,
    Backslash,
    Intersection,
}

#[derive(Clone)]
struct Cart {
    x: usize,
    y: usize,
    dir: u8,
    turn_counter: u8,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum BlockedState {
    Free,
    Blocked,
    Crashed,
}

#[derive(Clone)]
struct Map {
    tracks: Vec<Vec<Track>>,
    blocked: Vec<Vec<BlockedState>>,
}

impl Map {
    fn track(&self, x: usize, y: usize) -> Track {
        self.tracks[y][x]
    }

    fn get_blocked(&self, x: usize, y: usize) -> BlockedState {
        self.blocked[y][x]
    }

    fn set_blocked(&mut self, x: usize, y: usize, v: BlockedState) {
        self.blocked[y][x] = v;
    }
}

impl Cart {
    fn step(&mut self, map: &mut Map) -> bool {
        if map.get_blocked(self.x, self.y) == BlockedState::Crashed {
            return true;
        }
        let (new_x, new_y) = match self.dir {
            0 => (self.x, self.y - 1),
            1 => (self.x + 1, self.y),
            2 => (self.x, self.y + 1),
            3 => (self.x - 1, self.y),
            _ => unreachable!(),
        };
        map.set_blocked(self.x, self.y, BlockedState::Free);
        let crashed = map.get_blocked(new_x, new_y) != BlockedState::Free;
        map.set_blocked(
            new_x,
            new_y,
            if crashed {
                BlockedState::Crashed
            } else {
                BlockedState::Blocked
            },
        );
        self.x = new_x;
        self.y = new_y;
        self.dir = match map.track(self.x, self.y) {
            Track::None => return true,
            Track::Straight => self.dir,
            Track::Slash => self.dir ^ 1,
            Track::Backslash => self.dir ^ 3,
            Track::Intersection => {
                let new_dir = (self.dir + 3 + self.turn_counter) & 3;
                self.turn_counter = (self.turn_counter + 1) % 3;
                new_dir
            }
        };
        crashed
    }
}

fn parse_input(lines: &mut Iterator<Item = String>) -> Result<(Map, Vec<Cart>)> {
    let mut tracks = vec![];
    let mut blocked = vec![];
    let mut carts = vec![];
    for (y, line) in lines.enumerate() {
        let mut tracks_row = vec![];
        let mut blocked_row = vec![];
        for (x, c) in line.chars().enumerate() {
            let (track, blocked) = match c {
                ' ' => (Track::None, false),
                '-' | '|' => (Track::Straight, false),
                '/' => (Track::Slash, false),
                '\\' => (Track::Backslash, false),
                '+' => (Track::Intersection, false),
                '^' | '>' | 'v' | '<' => {
                    let dir = match c {
                        '^' => 0,
                        '>' => 1,
                        'v' => 2,
                        '<' => 3,
                        _ => unreachable!(),
                    };
                    carts.push(Cart {
                        x,
                        y,
                        dir,
                        turn_counter: 0,
                    });
                    (Track::Straight, true)
                }
                _ => bail!("Unexpected char '{}'", c),
            };
            tracks_row.push(track);
            blocked_row.push(if blocked {
                BlockedState::Blocked
            } else {
                BlockedState::Free
            });
        }
        tracks.push(tracks_row);
        blocked.push(blocked_row);
    }
    Ok((Map { tracks, blocked }, carts))
}

fn first_crash(mut map: Map, mut carts: Vec<Cart>) -> (usize, usize) {
    loop {
        carts.sort_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });
        for cart in carts.iter_mut() {
            if cart.step(&mut map) {
                return (cart.x, cart.y);
            }
        }
    }
}

fn last_alive(mut map: Map, mut carts: Vec<Cart>) -> (usize, usize) {
    let mut crashed: BTreeSet<(usize, usize)> = BTreeSet::new();
    while carts.len() > 0 {
        carts.sort_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });
        crashed.clear();
        for cart in carts.iter_mut() {
            if cart.step(&mut map) {
                crashed.insert((cart.x, cart.y));
            }
        }
        carts.retain(|c| !crashed.contains(&(c.x, c.y)));
        for &(x, y) in crashed.iter() {
            map.set_blocked(x, y, BlockedState::Free);
        }
        if carts.len() == 1 {
            return (carts[0].x, carts[0].y);
        }
    }
    (0, 0)
}

pub fn run(data: &AocData) -> AocResult {
    let (map, carts) = parse_input(&mut data.lines()?)?;

    let crash = first_crash(map.clone(), carts.clone());

    let last_alive = last_alive(map, carts);

    answers(crash, last_alive)
}
