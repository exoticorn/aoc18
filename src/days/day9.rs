use crate::prelude::*;
use std::ops::Index;

pub fn run(_: &AocData) -> AocResult {
    answers(marble_score(452, 71250), marble_score(452, 7125000))
}

struct Circle<T>(Vec<(usize, usize, T)>);

struct Cursor(usize);

impl<T> Circle<T> {
    fn new(v: T) -> (Circle<T>, Cursor) {
        (Circle(vec![(0, 0, v)]), Cursor(0))
    }

    fn insert_after(&mut self, cursor: Cursor, v: T) -> Cursor {
        let before = cursor.0;
        let after = self.0[before].1;
        let i = self.0.len();
        self.0[before].1 = i;
        self.0[after].0 = i;
        self.0.push((before, after, v));
        Cursor(i)
    }

    fn remove(&mut self, cursor: Cursor) -> Cursor {
        let i = cursor.0;
        let before = self.0[i].0;
        let after = self.0[i].1;
        self.0[before].1 = after;
        self.0[after].0 = before;
        Cursor(after)
    }

    fn step_forward(&self, cursor: Cursor) -> Cursor {
        Cursor(self.0[cursor.0].1)
    }

    fn step_backward(&self, cursor: Cursor) -> Cursor {
        Cursor(self.0[cursor.0].0)
    }
}

impl<T> Index<&Cursor> for Circle<T> {
    type Output = T;
    fn index(&self, c: &Cursor) -> &T {
        &self.0[c.0].2
    }
}

fn marble_score(num_players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0usize; num_players];
    let (mut circle, mut cursor) = Circle::new(0usize);
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                cursor = circle.step_backward(cursor);
            }
            let score = marble + circle[&cursor];
            cursor = circle.remove(cursor);
            scores[(marble - 1) % num_players] += score;
        } else {
            cursor = circle.insert_after(circle.step_forward(cursor), marble);
        }
    }
    scores.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(marble_score(9, 25), 32);
        assert_eq!(marble_score(10, 1618), 8317);
        assert_eq!(marble_score(13, 7999), 146373);
        assert_eq!(marble_score(17, 1104), 2764);
        assert_eq!(marble_score(21, 6111), 54718);
        assert_eq!(marble_score(30, 5807), 37305);
    }
}
