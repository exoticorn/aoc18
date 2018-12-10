use crate::prelude::*;
use std::cmp;
use std::i32;

struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Point {
    fn at_time(&self, s: i32) -> (i32, i32) {
        (self.pos.0 + self.vel.0 * s, self.pos.1 + self.vel.1 * s)
    }
}

struct BoundingRect {
    min: (i32, i32),
    max: (i32, i32),
}

impl BoundingRect {
    fn new() -> BoundingRect {
        BoundingRect {
            min: (i32::MAX, i32::MAX),
            max: (i32::MIN, i32::MIN),
        }
    }

    fn add(&mut self, p: (i32, i32)) {
        self.min.0 = cmp::min(self.min.0, p.0);
        self.min.1 = cmp::min(self.min.1, p.1);
        self.max.0 = cmp::max(self.max.0, p.0);
        self.max.1 = cmp::max(self.max.1, p.1);
    }

    fn area(&self) -> usize {
        self.width() * self.height()
    }

    fn width(&self) -> usize {
        cmp::max(0, self.max.0 - self.min.0) as usize + 1
    }

    fn height(&self) -> usize {
        cmp::max(0, self.max.1 - self.min.1) as usize + 1
    }
}

pub fn run(data: &AocData) -> AocResult {
    let mut points: Vec<Point> = Vec::new();
    let re =
        Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();

    for line in data.lines()? {
        if let Some(cap) = re.captures(&line) {
            points.push(Point {
                pos: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                vel: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            });
        } else {
            bail!("Failed to parse line: {}", line);
        }
    }

    let mut rect = BoundingRect::new();
    let mut time = 0;
    loop {
        let mut new_rect = BoundingRect::new();
        for p in &points {
            new_rect.add(p.at_time(time));
        }
        if time > 0 && rect.area() < new_rect.area() {
            break;
        }
        rect = new_rect;
        time += 1;
    }
    time -= 1;

    let mut s = vec!['.'; rect.area()];
    for p in &points {
        let (x, y) = p.at_time(time);
        s[(x - rect.min.0) as usize + (y - rect.min.1) as usize * rect.width()] = '#';
    }

    let mut result = String::with_capacity((rect.width() + 1) * rect.height() - 1);
    for y in 0..rect.height() {
        result.push('\n');
        let o = y * rect.width();
        result.extend(s[o..(o + rect.width())].iter().cloned());
    }

    answers(result, time)
}
