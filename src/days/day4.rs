use crate::prelude::*;
use std::collections::HashMap;
use std::ops::Range;

type Guards = HashMap<u32, Vec<Range<u32>>>;

fn parse_records<T: AsRef<str>>(records: &[T]) -> Result<Guards> {
    let mut guards: HashMap<u32, Vec<Range<u32>>> = HashMap::new();
    let re = Regex::new(r"\d\d:(\d\d)] (Guard #(\d+) begins shift|.+)$").unwrap();
    let mut guard_id: Option<u32> = None;
    let mut sleep_start: Option<u32> = None;
    for record in records {
        let record = record.as_ref();
        if let Some(captures) = re.captures(record) {
            if let Some(id) = captures.get(3) {
                guard_id = Some(id.as_str().parse().unwrap());
            } else {
                let time: u32 = captures[1].parse().unwrap();
                if time >= 60 {
                    bail!("Invalid minute value: {}", record);
                }
                let event = &captures[2];
                if event == "falls asleep" {
                    if sleep_start.is_some() {
                        bail!("Guard fell asleep twice??: {}", record);
                    }
                    {
                        sleep_start = Some(time);
                    }
                } else if event == "wakes up" {
                    if let Some(id) = guard_id {
                        if let Some(start) = sleep_start {
                            guards.entry(id).or_insert(Vec::new()).push(start..time);
                            sleep_start = None;
                        } else {
                            bail!("Wakes up without falling asleep first: {}", record);
                        }
                    } else {
                        bail!("Records don't start with guard beginning shift");
                    }
                }
            }
        } else {
            bail!("Failed to parse record '{}'", record);
        }
    }
    if guards.is_empty() {
        bail!("No guards found");
    }
    Ok(guards)
}

fn find_most_sleepy_guard(guards: &Guards) -> u32 {
    fn total_asleep(spans: &[Range<u32>]) -> u32 {
        spans.iter().map(|r| r.len() as u32).sum()
    }
    let mut guard_ids: Vec<_> = guards.keys().collect();
    guard_ids.sort_by(|a, b| total_asleep(&guards[b]).cmp(&total_asleep(&guards[a])));
    *guard_ids[0]
}

fn find_most_sleepy_minute(spans: &[Range<u32>]) -> (u32, usize) {
    let mut minutes = [0usize; 60];
    for span in spans {
        for minute in span.clone() {
            minutes[minute as usize] += 1;
        }
    }
    let mut max = 0;
    let mut minute = 0;
    for m in 0..60 {
        if minutes[m] > max {
            max = minutes[m];
            minute = m;
        }
    }
    (minute as u32, max)
}

fn find_most_likely_asleep(guards: &Guards) -> (u32, u32) {
    let mut max = 0;
    let mut best_guard = 0;
    let mut best_minute = 0;
    for (id, spans) in guards.iter() {
        let (minute, times_asleep) = find_most_sleepy_minute(spans);
        if times_asleep > max {
            max = times_asleep;
            best_guard = *id;
            best_minute = minute;
        }
    }
    (best_guard, best_minute)
}

pub fn run(data: &AocData) -> AocResult {
    let mut records: Vec<String> = data.lines()?.collect();
    records.sort();
    let guards = parse_records(&records)?;

    let most_sleepy_guard = find_most_sleepy_guard(&guards);
    let (most_sleepy_minute, _) = find_most_sleepy_minute(&guards[&most_sleepy_guard]);

    let (best_guard, best_minute) = find_most_likely_asleep(&guards);

    answers(
        most_sleepy_guard * most_sleepy_minute,
        best_guard * best_minute,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sleepy_guards() {
        let data = [
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ];

        let guards = parse_records(&data).unwrap();
        let most_sleepy_guard = find_most_sleepy_guard(&guards);
        assert_eq!(most_sleepy_guard, 10);
        let (most_sleepy_minute, _) = find_most_sleepy_minute(&guards[&10]);
        assert_eq!(most_sleepy_minute, 24);
        let (best_guard, best_minute) = find_most_likely_asleep(&guards);
        assert_eq!(best_guard, 99);
        assert_eq!(best_minute, 45);
    }
}
