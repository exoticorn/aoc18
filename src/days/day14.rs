use crate::prelude::*;

pub fn run(_: &AocData) -> AocResult {
    answers(scores_after(165061, 10), num_recipes_before("165061"))
}

struct Scoreboard {
    scores: Vec<u8>,
    i: usize,
    j: usize,
}

impl Scoreboard {
    fn new() -> Scoreboard {
        Scoreboard {
            scores: vec![3, 7],
            i: 0,
            j: 1,
        }
    }

    fn step(&mut self) {
        let sum = self.scores[self.i] + self.scores[self.j];
        if sum >= 10 {
            self.scores.push(sum / 10);
        }
        self.scores.push(sum % 10);
        self.i = (self.i + self.scores[self.i] as usize + 1) % self.scores.len();
        self.j = (self.j + self.scores[self.j] as usize + 1) % self.scores.len();
    }

    fn scores(&self) -> &[u8] {
        &self.scores
    }
}

fn scores_after(after_num: usize, count: usize) -> String {
    let mut scoreboard = Scoreboard::new();
    while scoreboard.scores().len() < after_num + count {
        scoreboard.step();
    }
    scoreboard.scores()[after_num..(after_num + count)]
        .iter()
        .map(|s| (s + 48) as char)
        .collect()
}

fn num_recipes_before(seq: &str) -> usize {
    let seq: Vec<u8> = seq.chars().map(|c| c as u8 - 48).collect();
    let mut i = 0;
    let mut scoreboard = Scoreboard::new();
    loop {
        while scoreboard.scores().len() < i + seq.len() {
            scoreboard.step();
        }
        if &scoreboard.scores()[i..(i + seq.len())] == seq.as_slice() {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(scores_after(9, 10), "5158916779");
    assert_eq!(scores_after(5, 10), "0124515891");
    assert_eq!(scores_after(2018, 10), "5941429882");

    assert_eq!(num_recipes_before("59414"), 2018);
}
