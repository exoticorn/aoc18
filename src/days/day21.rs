use crate::prelude::*;

pub fn run(_: &AocData) -> AocResult {
    let shortest = native(1);
    assert_eq!(native_opt(shortest), 1);
    assert_eq!(native_opt(native(1234)), 1234);
    let (longest_cycle, longest_r0) = find_cycle();
    assert_eq!(native_opt(longest_r0), longest_cycle);
    answers(shortest, longest_r0)
}

fn native(mut num_iterations: usize) -> usize {
    let mut r1: usize;
    let mut r2: usize;
    let mut r3: usize;
    let mut r5: usize;
    // #ip 4
    // 00  seti 123 0 1
    r1 = 123;
    // 01  bani 1 456 1
    r1 &= 456;
    // 02  eqri 1 72 1
    r1 = (r1 == 72) as usize;
    // 03  addr 1 4 4
    // 04  seti 0 0 4
    if r1 == 0 {
        panic!("infinite loop")
    }
    // 05  seti 0 3 1
    r1 = 0;
    // LABEL E
    while num_iterations > 0 {
        num_iterations -= 1;
        // 06  bori 1 65536 2
        r2 = r1 | 65536;
        // 07  seti 7902108 7 1
        r1 = 7902108;
        // LABEL D
        loop {
            // 08  bani 2 255 5
            r5 = r2 & 255;
            // 09  addr 1 5 1
            r1 += r5;
            // 10  bani 1 16777215 1
            r1 &= 16777215;
            // 11  muli 1 65899 1
            r1 *= 65899;
            // 12  bani 1 16777215 1
            r1 &= 16777215;
            // 13  gtir 256 2 5
            r5 = (256 > r2) as usize;
            // 14  addr 5 4 4
            // 15  addi 4 1 4
            // 16  seti 27 0 4
            if r5 == 1 {
                // r4 = 27; // goto LABEL A
                // 17  seti 0 0 5
                break;
            }
            r5 = 0;
            // LABEL C
            loop {
                // 18  addi 5 1 3
                r3 = r5 + 1;
                // 19  muli 3 256 3
                r3 *= 256;
                // 20  gtrr 3 2 3]
                r3 = (r3 > r2) as usize;
                // 21  addr 3 4 4
                // 22  addi 4 1 4
                // 23  seti 25 2 4
                if r3 != 0 {
                    // r4 = 25; // goto LABEL B
                    // 24  addi 5 1 5
                    break;
                }
                r5 += 1;
                // 25  seti 17 2 4
                // r4 = 17; // goto LABEL C
            }
            // LABEL B
            // 26  setr 5 1 2
            r2 = r5;
            // 27  seti 7 2 4
            // r4 = 7; // goto LABEL D
        }
        // LABEL A
        // 28  eqrr 1 0 5
        // r5 = (r1 == r0) as usize;
        // 29  addr 5 4 4
        // 30  seti 5 9 4
        if r5 != 0 {
            // break;
        }
    }
    r1
}

fn native_opt(r0: usize) -> usize {
    let mut r1: usize;
    let mut r2: usize;
    let mut num_iterations = 0;
    r1 = 0;
    loop {
        num_iterations += 1;
        r2 = r1 | 0x10000;
        r1 = 7902108;
        while r2 > 0 {
            r1 = (r1 + (r2 & 0xff)) & 0xffffff;
            r1 = (r1 * 65899) & 0xffffff;
            r2 /= 256;
        }
        if r1 == r0 {
            break;
        }
    }
    num_iterations
}

fn find_cycle() -> (usize, usize) {
    let mut visited = vec![false; 0x1000000];
    let mut num_iterations = 0;
    let mut rng = 0usize;
    let mut prev_rng = 0usize;
    loop {
        let mut seed = rng | 0x10000;
        rng = 7902108;
        while seed > 0 {
            rng = (rng + (seed & 0xff)) & 0xffffff;
            rng = (rng * 65899) & 0xffffff;
            seed /= 256;
        }
        if visited[rng] {
            return (num_iterations, prev_rng);
        }
        visited[rng] = true;
        prev_rng = rng;
        num_iterations += 1;
    }
}
