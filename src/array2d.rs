#![allow(dead_code)]

use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Array2d<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2d<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy,
    {
        Self::new_value(width, height, T::default())
    }

    pub fn new_value(width: usize, height: usize, v: T) -> Self
    where
        T: Copy,
    {
        Self::new_with(width, height, |_, _| v)
    }

    pub fn new_with<F>(width: usize, height: usize, f: F) -> Self
    where
        F: Fn(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y));
            }
        }
        Array2d {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T
    where
        T: Default + Copy,
    {
        if x < self.width && y < self.height {
            self.data[x + y * self.width]
        } else {
            T::default()
        }
    }

    pub fn get_opt(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[x + y * self.width])
        } else {
            None
        }
    }

    pub fn put(&mut self, x: usize, y: usize, v: T) {
        if x < self.width && y < self.height {
            self.data[x + y * self.width] = v;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, v: T)
    where
        T: Copy,
    {
        for c in self.data.iter_mut() {
            *c = v;
        }
    }

    pub fn fill_with<F>(&mut self, f: F)
    where
        F: Fn(usize, usize) -> T,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                self.data[x + y * self.width] = f(x, y);
            }
        }
    }

    pub fn iter_xy(&self) -> IterXY {
        IterXY {
            x: 0,
            y: 0,
            w: self.width,
            h: self.height,
        }
    }

    pub fn to_string<F>(&self, f: F) -> String
    where
        F: Fn(&T) -> char,
    {
        if self.width == 0 && self.height == 0 {
            return String::new();
        }
        let mut s = String::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height() {
            if y > 0 {
                s.push('\n');
            }
            for x in 0..self.width() {
                s.push(f(&self.data[x + y * self.width()]));
            }
        }
        s
    }
}

impl<T> Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        if x < self.width && y < self.height {
            &self.data[x + y * self.width]
        } else {
            panic!("Index out of range: {}, {}", x, y);
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        if x < self.width && y < self.height {
            &mut self.data[x + y * self.width]
        } else {
            panic!("Index out of range: {}, {}", x, y);
        }
    }
}

pub struct IterXY {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Iterator for IterXY {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let (x, y) = (self.x, self.y);
        self.x += 1;
        if self.x == self.w {
            self.x = 0;
            self.y += 1;
        }
        if y < self.h {
            Some((x, y))
        } else {
            None
        }
    }
}
