use crate::prelude::*;
use std::cmp;
use std::ops::Range;
use std::{isize, u32};

struct Grid {
    x0: isize,
    y0: isize,
    width: isize,
    height: isize,
    data: Vec<(u32, u32)>,
}

impl Grid {
    fn new(x0: isize, y0: isize, width: isize, height: isize) -> Grid {
        assert!(width > 0 && height > 0);
        Grid {
            x0,
            y0,
            width,
            height,
            data: vec![(0, 0); width as usize * height as usize],
        }
    }

    fn inside(&self, x: isize, y: isize) -> bool {
        x >= self.x0 && x < self.x0 + self.width && y >= self.y0 && y < self.y0 + self.height
    }

    fn get(&self, x: isize, y: isize) -> (u32, u32) {
        if self.inside(x, y) {
            self.data[(x - self.x0 + (y - self.y0) * self.width) as usize]
        } else {
            (0, 0)
        }
    }

    fn set(&mut self, x: isize, y: isize, v: (u32, u32)) {
        if self.inside(x, y) {
            self.data[(x - self.x0 + (y - self.y0) * self.width) as usize] = v;
        }
    }

    fn minx(&self) -> isize {
        self.x0
    }
    fn maxx(&self) -> isize {
        self.x0 + self.width - 1
    }
    fn miny(&self) -> isize {
        self.y0
    }
    fn maxy(&self) -> isize {
        self.y0 + self.height - 1
    }
    fn xs(&self) -> Range<isize> {
        self.x0..(self.x0 + self.width)
    }
    fn ys(&self) -> Range<isize> {
        self.y0..(self.y0 + self.height)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.ys() {
            for x in self.xs() {
                let (id, _) = self.get(x, y);
                if id == 0 {
                    print!("_");
                } else if id == u32::MAX {
                    print!(".");
                } else {
                    print!("{}", (id + 96) as u8 as char);
                }
            }
            println!();
        }
        println!();
    }
}

fn parse_coords(data: &mut dyn Iterator<Item = String>) -> Result<Vec<(isize, isize)>> {
    let re = Regex::new(r"^(\d+)\s*,\s*(\d+)$").unwrap();
    let mut coords = Vec::new();
    for line in data {
        if let Some(captures) = re.captures(&line) {
            let x: isize = captures[1].parse().unwrap();
            let y: isize = captures[2].parse().unwrap();
            coords.push((x, y));
        } else {
            bail!("Failed to parse coord line: {}", line);
        }
    }
    if coords.is_empty() {
        bail!("No coordinates in input");
    }
    Ok(coords)
}

type Coords = [(isize, isize)];

fn fill_grid(coords: &Coords) -> Grid {
    let (mut minx, mut maxx, mut miny, mut maxy) = (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for &(x, y) in coords {
        minx = cmp::min(minx, x);
        maxx = cmp::max(maxx, x);
        miny = cmp::min(miny, y);
        maxy = cmp::max(maxy, y);
    }

    let mut grid = Grid::new(minx, miny, maxx - minx + 1, maxy - miny + 1);

    for (index, &(x, y)) in coords.iter().enumerate() {
        grid.set(x, y, (index as u32 + 1, 0));
    }

    fn try_set(grid: &mut Grid, x: isize, y: isize, id: u32, distance: u32) -> bool {
        if !grid.inside(x, y) {
            return false;
        }
        let (prev_id, prev_distance) = grid.get(x, y);
        if prev_id != id && (prev_id == 0 || distance <= prev_distance) {
            let new_id = if distance > prev_distance {
                id
            } else {
                u32::MAX
            };
            grid.set(x, y, (new_id, distance));
            true
        } else {
            false
        }
    }

    let mut iter = 0u32;
    loop {
        let mut changed = false;
        for y in grid.ys() {
            for x in grid.xs() {
                let (id, distance) = grid.get(x, y);
                if id > 0 && distance == iter {
                    changed |= try_set(&mut grid, x - 1, y, id, distance + 1);
                    changed |= try_set(&mut grid, x + 1, y, id, distance + 1);
                    changed |= try_set(&mut grid, x, y - 1, id, distance + 1);
                    changed |= try_set(&mut grid, x, y + 1, id, distance + 1);
                }
            }
        }
        iter += 1;
        if !changed {
            break;
        }
    }

    grid
}

fn count_areas(grid: &Grid) -> Vec<usize> {
    let mut counts: Vec<usize> = Vec::new();
    for y in grid.ys() {
        for x in grid.xs() {
            let (id, _) = grid.get(x, y);
            if id != 0 && id != u32::MAX {
                if counts.len() < id as usize {
                    counts.resize(id as usize, 0);
                }
                counts[(id - 1) as usize] += 1;
            }
        }
    }

    fn invalid(counts: &mut [usize], grid: &Grid, x: isize, y: isize) {
        let (id, _) = grid.get(x, y);
        if id != 0 && id != u32::MAX {
            counts[(id - 1) as usize] = 0;
        }
    }

    for y in grid.ys() {
        invalid(&mut counts, grid, grid.minx(), y);
        invalid(&mut counts, grid, grid.maxx(), y);
    }

    for x in grid.xs() {
        invalid(&mut counts, grid, x, grid.miny());
        invalid(&mut counts, grid, x, grid.maxy());
    }

    counts
}

fn distance(coords: &Coords, x: isize, y: isize) -> usize {
    let mut dist = 0usize;
    for &(cx, cy) in coords {
        dist += (x - cx).abs() as usize + (y - cy).abs() as usize
    }
    dist
}

fn find_point_inside(coords: &Coords, max_distance: usize) -> Result<(isize, isize)> {
    for &(x, y) in coords {
        if distance(coords, x, y) < max_distance {
            return Ok((x, y));
        }
    }
    bail!("Failed to find point inside safe area");
}

fn find_inside_span(
    coords: &Coords,
    y: isize,
    seed: Range<isize>,
    max_distance: usize,
) -> Range<isize> {
    let mut left = seed.start;
    let mut right = seed.end - 1;

    if distance(coords, left, y) < max_distance {
        while distance(coords, left - 1, y) < max_distance {
            left -= 1;
        }
    } else {
        left += 1;
        while left <= right && distance(coords, left, y) >= max_distance {
            left += 1;
        }
    }

    if distance(coords, right, y) < max_distance {
        while distance(coords, right + 1, y) < max_distance {
            right += 1;
        }
    } else {
        right -= 1;
        while right >= left && distance(coords, right, y) >= max_distance {
            right -= 1;
        }
    }

    left..cmp::max(left, right + 1)
}

fn find_inside_area(coords: &Coords, max_distance: usize) -> Result<usize> {
    let (sx, sy) = find_point_inside(coords, max_distance)?;

    let seed = find_inside_span(coords, sy, sx..(sx + 1), max_distance);
    let mut sum = seed.len();

    let mut span = seed.clone();
    let mut y = sy;
    while span.len() > 0 {
        y -= 1;
        span = find_inside_span(coords, y, span, max_distance);
        sum += span.len();
    }

    let mut span = seed;
    let mut y = sy;
    while span.len() > 0 {
        y += 1;
        span = find_inside_span(coords, y, span, max_distance);
        sum += span.len();
    }

    Ok(sum)
}

pub fn run(data: &AocData) -> AocResult {
    let coords = parse_coords(&mut data.lines()?)?;
    let grid = fill_grid(&coords);
    let areas = count_areas(&grid);

    answers(
        areas.into_iter().max().unwrap(),
        find_inside_area(&coords, 10000)?,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let data = &["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        let coords = parse_coords(&mut data.iter().map(|s| s.to_string())).unwrap();
        let grid = fill_grid(&coords);
        let areas = count_areas(&grid);
        assert_eq!(areas.into_iter().max().unwrap(), 17);
        assert_eq!(find_inside_area(&coords, 32).unwrap(), 16);
    }
}
