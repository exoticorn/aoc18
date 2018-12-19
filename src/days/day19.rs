use crate::prelude::*;

pub fn run(data: &AocData) -> AocResult {
    let mut proc1 = Machine::parse(data)?;
    while proc1.step() {}

    assert_eq!(native(0) as usize, proc1.reg[0]);
    answers(proc1.reg[0], native(1))
}

fn native(mut r0: u32) -> u32 {
    let mut r1;
    let mut r2;
    let mut r5;

    r2 = 131;
    r5 = r2 + 836;
    if r0 == 1 {
        r2 = 10550400;
        r5 += r2;
        r0 = 0;
    }

    r1 = 1;
    loop {
        if r5 % r1 == 0 {
            r0 += r1;
            let other = r5 / r1;
            if other != r1 {
                r0 += other;
            }
        }
        r1 += 1;
        if r1 * r1 > r5 {
            break;
        }
    }

    r0
}

#[derive(Clone, Copy)]
enum Op {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

struct Inst {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

struct Machine {
    reg: [usize; 6],
    pc: usize,
    code: Vec<Inst>,
}

impl Machine {
    fn parse(data: &AocData) -> Result<Machine> {
        let mut pc = 0;
        let mut code = vec![];

        let re = Regex::new(r"^(addr|addi|mulr|muli|banr|bani|borr|bori|setr|seti|gtir|gtri|gtrr|eqir|eqri|eqrr) (\d+) (\d+) (\d+)$").unwrap();

        for line in data.lines()? {
            if line.starts_with("#ip ") {
                pc = line[4..].parse()?;
            } else if let Some(caps) = re.captures(&line) {
                let a = caps[2].parse().unwrap();
                let b = caps[3].parse().unwrap();
                let c = caps[4].parse().unwrap();
                fn r(v: usize) -> Result<usize> {
                    if v >= 6 {
                        bail!("Register out of range: {}", v);
                    }
                    Ok(v)
                }
                let (op, a, b, c) = match &caps[1] {
                    "addr" => (Op::AddR, r(a)?, r(b)?, r(c)?),
                    "addi" => (Op::AddI, r(a)?, b, r(c)?),
                    "mulr" => (Op::MulR, r(a)?, r(b)?, r(c)?),
                    "muli" => (Op::MulI, r(a)?, b, r(c)?),
                    "banr" => (Op::BanR, r(a)?, r(b)?, r(c)?),
                    "bani" => (Op::BanI, r(a)?, b, r(c)?),
                    "borr" => (Op::BorR, r(a)?, r(b)?, r(c)?),
                    "bori" => (Op::BorI, r(a)?, b, r(c)?),
                    "setr" => (Op::SetR, r(a)?, b, r(c)?),
                    "seti" => (Op::SetI, a, b, r(c)?),
                    "gtir" => (Op::GtIR, a, r(b)?, r(c)?),
                    "gtri" => (Op::GtRI, r(a)?, b, r(c)?),
                    "gtrr" => (Op::GtRR, r(a)?, r(b)?, r(c)?),
                    "eqir" => (Op::EqIR, a, r(b)?, r(c)?),
                    "eqri" => (Op::EqRI, r(a)?, b, r(c)?),
                    "eqrr" => (Op::EqRR, r(a)?, r(b)?, r(c)?),
                    _ => unreachable!(),
                };
                code.push(Inst { op, a, b, c })
            } else {
                bail!("Failed to parse line: {}", line)
            }
        }

        Ok(Machine {
            reg: [0; 6],
            pc,
            code,
        })
    }

    fn step(&mut self) -> bool {
        let r = &mut self.reg;
        let pc = r[self.pc];
        if pc >= self.code.len() {
            return false;
        }

        let i = &self.code[pc];

        match i.op {
            Op::AddR => r[i.c] = r[i.a] + r[i.b],
            Op::AddI => r[i.c] = r[i.a] + i.b,
            Op::MulR => r[i.c] = r[i.a] * r[i.b],
            Op::MulI => r[i.c] = r[i.a] * i.b,
            Op::BanR => r[i.c] = r[i.a] & r[i.b],
            Op::BanI => r[i.c] = r[i.a] & i.b,
            Op::BorR => r[i.c] = r[i.a] | r[i.b],
            Op::BorI => r[i.c] = r[i.a] | i.b,
            Op::SetR => r[i.c] = r[i.a],
            Op::SetI => r[i.c] = i.a,
            Op::GtIR => r[i.c] = (i.a > r[i.b]) as usize,
            Op::GtRI => r[i.c] = (r[i.a] > i.b) as usize,
            Op::GtRR => r[i.c] = (r[i.a] > r[i.b]) as usize,
            Op::EqIR => r[i.c] = (i.a == r[i.b]) as usize,
            Op::EqRI => r[i.c] = (r[i.a] == i.b) as usize,
            Op::EqRR => r[i.c] = (r[i.a] == r[i.b]) as usize,
        }

        r[self.pc] += 1;

        return true;
    }
}
