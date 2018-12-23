use crate::array2d::DynArray2d;
use crate::prelude::*;
use std::cell::RefCell;
use std::cmp;
use std::iter;
use std::u32;

pub fn run(_: &AocData) -> AocResult {
    let cave = Cave::new(7305, 13, 734);
    let risk_level = cave.area_risk_level();
    let mut path_finder = PathFinder::new(cave);
    answers(risk_level, path_finder.find_path())
}

struct Cave {
    tx: usize,
    ty: usize,
    depth: u32,
    erosion_level: RefCell<Vec<Vec<u32>>>,
}

impl Cave {
    fn new(depth: u32, tx: usize, ty: usize) -> Cave {
        Cave {
            tx,
            ty,
            depth,
            erosion_level: RefCell::new(Vec::new()),
        }
    }

    fn erosion_level(&self, x: usize, y: usize) -> u32 {
        let cache = self.erosion_level.borrow();
        if cache.len() > y {
            if cache[y].len() > x {
                return cache[y][x];
            }
        }
        drop(cache);

        let u = if y > 0 {
            self.erosion_level(x, y - 1)
        } else {
            0
        };

        let l = if x > 0 {
            self.erosion_level(x - 1, y)
        } else {
            0
        };

        let i = if (x == 0 && y == 0) || (x == self.tx && y == self.ty) {
            0
        } else if y == 0 {
            (x * 16807) as u32
        } else if x == 0 {
            (y * 48271) as u32
        } else {
            u * l
        };
        let i = (i + self.depth) % 20183;
        let mut cache = self.erosion_level.borrow_mut();
        if cache.len() <= y {
            cache.push(vec![i]);
        } else {
            cache[y].push(i);
        }
        i
    }

    fn risk_level(&self, x: usize, y: usize) -> u32 {
        self.erosion_level(x, y) % 3
    }

    fn area_risk_level(&self) -> u32 {
        (0..=self.ty)
            .map(|y| (0..=self.tx).map(|x| self.risk_level(x, y)).sum::<u32>())
            .sum()
    }
}

struct PathMapNode(u32);

impl Default for PathMapNode {
    fn default() -> PathMapNode {
        PathMapNode(u32::MAX)
    }
}

type PathMap = DynArray2d<[PathMapNode; 3]>;

struct PathMapPending {
    x: usize,
    y: usize,
    equip: u8,
    min_time: u32,
}

struct PathFinder {
    cave: Cave,
    map: PathMap,
    pending: Vec<PathMapPending>,
    min_pushed: u32,
    read_index: usize,
}

impl PathFinder {
    fn new(cave: Cave) -> PathFinder {
        PathFinder {
            cave,
            map: PathMap::new(),
            pending: Vec::new(),
            min_pushed: u32::MAX,
            read_index: 0,
        }
    }

    fn add_node(&mut self, x: usize, y: usize, equip: u8, time: u32) {
        let tpe = self.cave.risk_level(x, y) as u8;
        if tpe == equip {
            return;
        }
        let node = &mut self.map.get_mut(x, y)[equip as usize];
        if node.0 > time {
            node.0 = time;
            let mut min_time = time
                + (x as isize - self.cave.tx as isize).abs() as u32
                + (y as isize - self.cave.ty as isize).abs() as u32;
            if equip != 1 {
                min_time += 7;
            }
            self.pending.push(PathMapPending {
                x,
                y,
                equip,
                min_time,
            });
            self.min_pushed = cmp::min(self.min_pushed, min_time);
        }
    }

    fn find_path(&mut self) -> u32 {
        self.add_node(0, 0, 1, 0);
        loop {
            let head_time = self.pending[self.read_index].min_time;
            if head_time > self.min_pushed {
                self.pending
                    .splice(..self.read_index, iter::empty())
                    .for_each(|_| ());
                self.read_index = 0;
                self.pending.sort_by(|a, b| a.min_time.cmp(&b.min_time));
                self.min_pushed = u32::MAX;
            }
            let PathMapPending { x, y, equip, .. } = self.pending[self.read_index];
            self.read_index += 1;
            let time = self.map.get_mut(x, y)[equip as usize].0;
            if x == self.cave.tx && y == self.cave.ty && equip == 1 {
                return time;
            }
            if x > 0 {
                self.add_node(x - 1, y, equip, time + 1);
            }
            if y > 0 {
                self.add_node(x, y - 1, equip, time + 1);
            }
            self.add_node(x + 1, y, equip, time + 1);
            self.add_node(x, y + 1, equip, time + 1);
            let tpe = self.cave.risk_level(x, y) as u8;
            let mut equip = (equip + 1) % 3;
            if equip == tpe {
                equip = (equip + 1) % 3
            };
            self.add_node(x, y, equip, time + 7);
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let cave = Cave::new(510, 10, 10);
    assert_eq!(cave.risk_level(0, 0), 0);
    assert_eq!(cave.risk_level(1, 0), 1);
    assert_eq!(cave.risk_level(0, 1), 0);
    assert_eq!(cave.risk_level(1, 1), 2);
    assert_eq!(cave.area_risk_level(), 114);

    let mut path_finder = PathFinder::new(cave);
    assert_eq!(path_finder.find_path(), 45);
}
