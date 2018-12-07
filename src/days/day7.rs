use crate::prelude::*;

fn parse_dependencies(lines: &mut dyn Iterator<Item = String>) -> Result<(u32, [u32; 26])> {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let mut steps = 0u32;
    let mut deps = [0u32; 26];
    for line in lines {
        if let Some(captures) = re.captures(&line) {
            let dep = captures[1].chars().next().unwrap() as u32 - 65;
            let step = captures[2].chars().next().unwrap() as u32 - 65;
            steps |= 1u32 << step;
            steps |= 1u32 << dep;
            deps[step as usize] |= 1u32 << dep;
        } else {
            bail!("Failed to parse line: {}", line);
        }
    }
    Ok((steps, deps))
}

fn compute_order(steps: u32, deps: &[u32; 26]) -> Result<String> {
    let mut completed = 0u32;
    let mut result = String::new();
    while completed != steps {
        let step = (0..26).find(|&s| {
            (steps & !completed) & (1u32 << s) > 0
                && completed & deps[s as usize] == deps[s as usize]
        });
        if let Some(step) = step {
            result.push((step as u8 + 65) as char);
            completed |= 1u32 << step;
        } else {
            bail!("Can't make progress after steps: {}", result);
        }
    }
    Ok(result)
}

fn simulate(steps: u32, deps: &[u32; 26], num_workers: usize, base_time: usize) -> usize {
    let base_time = base_time + 1;
    let mut completed = 0u32;
    let mut taken = 0u32;
    let mut clock = 0usize;
    let mut workers = vec![(0u32, 0usize); num_workers];

    while completed != steps {
        for &mut (ref mut task, ref mut end_time) in workers.iter_mut() {
            if *task == 0 {
                if let Some(step) = (0..26).find(|&s| {
                    (steps & !taken) & (1u32 << s) > 0
                        && completed & deps[s as usize] == deps[s as usize]
                }) {
                    *task = 1u32 << step;
                    taken |= *task;
                    *end_time = clock + step as usize + base_time;
                }
            }
        }

        clock += 1;

        for &mut (ref mut task, end_time) in workers.iter_mut() {
            if *task != 0 && end_time == clock {
                completed |= *task;
                *task = 0;
            }
        }
    }

    clock
}

pub fn run(data: &AocData) -> AocResult {
    let (steps, deps) = parse_dependencies(&mut data.lines()?)?;
    let order = compute_order(steps, &deps)?;

    let time = simulate(steps, &deps, 5, 60);

    answers(order, time)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let (steps, deps) = parse_dependencies(
            &mut [
                "Step C must be finished before step A can begin.",
                "Step C must be finished before step F can begin.",
                "Step A must be finished before step B can begin.",
                "Step A must be finished before step D can begin.",
                "Step B must be finished before step E can begin.",
                "Step D must be finished before step E can begin.",
                "Step F must be finished before step E can begin.",
            ]
            .into_iter()
            .map(|s| s.to_string()),
        )
        .unwrap();
        let time = simulate(steps, &deps, 2, 0);
        assert_eq!(time, 15);
    }
}
