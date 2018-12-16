use crate::prelude::*;
use std::mem;

pub fn run(data: &AocData) -> AocResult {
    let (samples, insts) = parse_input(data)?;
    let part1 = samples
        .iter()
        .filter(|s| count_matching_ops(s) >= 3)
        .count();

    let mapping = create_opcode_map(&samples)?;
    let result = run_with_mapping(&insts, mapping);

    answers(part1, result[0])
}

struct Inst {
    op: u8,
    a: u8,
    b: u8,
    c: u8,
}
type Registers = [u32; 4];

struct Sample {
    before: Registers,
    inst: Inst,
    after: Registers,
}

enum ParseState {
    Idle,
    Before(Registers),
    Inst(Registers, Inst),
}

fn parse_input(data: &AocData) -> Result<(Vec<Sample>, Vec<Inst>)> {
    let reg_re = Regex::new(r"^(Before|After):\s*\[(\d), (\d), (\d), (\d)]$").unwrap();
    let inst_re = Regex::new(r"^(\d+) (\d) (\d) (\d)$").unwrap();
    let mut state = ParseState::Idle;
    let mut samples: Vec<Sample> = Vec::new();
    let mut insts: Vec<Inst> = Vec::new();
    for line in data.lines()? {
        if line.is_empty() {
            continue;
        }
        if let Some(caps) = reg_re.captures(&line) {
            let regs: [u32; 4] = [
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
                caps[5].parse().unwrap(),
            ];
            if &caps[1] == "Before" {
                if let ParseState::Idle = mem::replace(&mut state, ParseState::Idle) {
                    state = ParseState::Before(regs);
                } else {
                    bail!("invalid parse state");
                }
            } else {
                if let ParseState::Inst(before, inst) = mem::replace(&mut state, ParseState::Idle) {
                    samples.push(Sample {
                        before,
                        inst,
                        after: regs,
                    });
                } else {
                    bail!("Invalid parse state");
                }
            }
        } else if let Some(caps) = inst_re.captures(&line) {
            let inst = Inst {
                op: caps[1].parse().unwrap(),
                a: caps[2].parse().unwrap(),
                b: caps[3].parse().unwrap(),
                c: caps[4].parse().unwrap(),
            };
            match mem::replace(&mut state, ParseState::Idle) {
                ParseState::Idle => insts.push(inst),
                ParseState::Before(before) => {
                    state = ParseState::Inst(before, inst);
                }
                _ => bail!("Invalid parse state"),
            }
        } else {
            bail!("Failed to parse line: {}", line)
        }
    }
    Ok((samples, insts))
}

fn exec_op(op: u8, inst: &Inst, regs: &mut Registers) {
    let ra = (inst.a & 3) as usize;
    let rb = (inst.b & 3) as usize;
    let rc = (inst.c & 3) as usize;
    let va = inst.a as u32;
    let vb = inst.b as u32;
    match op {
        0 => regs[rc] = regs[ra] + regs[rb],
        1 => regs[rc] = regs[ra] + vb,
        2 => regs[rc] = regs[ra] * regs[rb],
        3 => regs[rc] = regs[ra] * vb,
        4 => regs[rc] = regs[ra] & regs[rb],
        5 => regs[rc] = regs[ra] & vb,
        6 => regs[rc] = regs[ra] | regs[rb],
        7 => regs[rc] = regs[ra] | vb,
        8 => regs[rc] = regs[ra],
        9 => regs[rc] = va,
        10 => regs[rc] = (regs[ra] > regs[rb]) as u32,
        11 => regs[rc] = (regs[ra] > vb) as u32,
        12 => regs[rc] = (va > regs[rb]) as u32,
        13 => regs[rc] = (regs[ra] == regs[rb]) as u32,
        14 => regs[rc] = (regs[ra] == vb) as u32,
        15 => regs[rc] = (va == regs[rb]) as u32,
        _ => panic!("Bad opcode: {}", op),
    }
}

fn count_matching_ops(sample: &Sample) -> u32 {
    matching_ops(sample).count_ones()
}

fn matching_ops(sample: &Sample) -> u16 {
    let mut result = 0;
    for i in 0..16 {
        let mut regs = sample.before.clone();
        exec_op(i, &sample.inst, &mut regs);
        if regs == sample.after {
            result |= 1 << i;
        }
    }
    result
}

fn create_opcode_map(samples: &[Sample]) -> Result<[u8; 16]> {
    let mut candidates = [65535u16; 16];
    for sample in samples {
        candidates[sample.inst.op as usize] &= matching_ops(sample);
    }
    let mut result = [0u8; 16];
    let mut used = 0u16;
    for _ in 0..16 {
        for i in 0..16 {
            let mask = candidates[i] & !used;
            if mask.count_ones() == 1 {
                used |= mask;
                result[i] = mask.trailing_zeros() as u8;
            }
        }
    }
    if used != 65535 {
        bail!("No unique mapping found: {:b}", used);
    }
    Ok(result)
}

fn run_with_mapping(insts: &[Inst], mapping: [u8; 16]) -> Registers {
    let mut regs = [0, 0, 0, 0];
    for inst in insts {
        exec_op(mapping[inst.op as usize], inst, &mut regs);
    }
    regs
}
