use crate::prelude::*;

pub fn run(_: &AocData) -> AocResult {
    answers(best_square(1723), best_square_any_size(1723))
}

fn power_level(grid_serial: u32, x: u32, y: u32) -> i32 {
    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + grid_serial;
    let power = power * rack_id;
    let power = (power / 100) % 10;
    power as i32 - 5
}

fn best_square(grid_serial: u32) -> (u32, u32) {
    let mut buffer = vec![];
    let (x, y, _) = find_best_square(grid_serial, 3, &mut buffer);
    (x, y)
}

fn find_best_square(grid_serial: u32, size: u32, buffer: &mut Vec<i32>) -> (u32, u32, i32) {
    buffer.clear();
    let buffer_width = (301 - size) as usize;
    for y in 1..=301 {
        let mut p: i32 = (1..size).map(|x| power_level(grid_serial, x, y)).sum();
        for x in 1..=(301 - size) {
            p += power_level(grid_serial, x + size - 1, y);
            buffer.push(p);
            p -= power_level(grid_serial, x, y);
        }
    }
    (1..=(301 - size))
        .map(|x| {
            let xo = x as usize - 1;
            let mut p: i32 = (1..size)
                .map(|y| buffer[xo + (y as usize - 1) * buffer_width])
                .sum();
            (1..=(301 - size))
                .map(|y| {
                    p += buffer[xo + (y + size - 2) as usize * buffer_width];
                    let power = p;
                    p -= buffer[xo + (y as usize - 1) * buffer_width];
                    (x, y, power)
                })
                .max_by_key(|&(_, _, p)| p)
                .unwrap()
        })
        .max_by_key(|&(_, _, p)| p)
        .unwrap()
}

fn best_square_any_size(grid_serial: u32) -> (u32, u32, u32) {
    let mut buffer = vec![];
    let (x, y, _, size) = (1..=300)
        .map(|size| {
            let (x, y, p) = find_best_square(grid_serial, size, &mut buffer);
            (x, y, p, size)
        })
        .max_by_key(|&(_, _, p, _)| p)
        .unwrap();
    (x, y, size)
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(power_level(8, 3, 5), 4);
    assert_eq!(power_level(57, 122, 79), -5);
    assert_eq!(power_level(39, 217, 196), 0);
    assert_eq!(power_level(71, 101, 153), 4);

    assert_eq!(best_square(18), (33, 45));

    assert_eq!(best_square_any_size(18), (90, 269, 16));
    assert_eq!(best_square_any_size(42), (232, 251, 12));
}
