use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_from_file() -> Vec<Step> {
    let file = File::open("input/day07.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| parse_step(line.unwrap()))
        .collect()
}

#[derive(Debug)]
struct Step {
    before: char,
    after: char,
}

fn parse_step(line: String) -> Step {
    // Step C must be finished before step A can begin.
    let bytes: Vec<char> = line.chars().collect();
    Step {
        before: bytes[5],
        after: bytes[36],
    }
}

pub fn a() -> String {
    _a(load_from_file())
}

pub fn b() -> u64 {
    _b(load_from_file(), 60, 5)
}

fn _a(steps: Vec<Step>) -> String {
    let mut set: HashMap<char, HashSet<char>> = HashMap::new();

    for step in steps.iter() {
        let entry = set.entry(step.after as char).or_insert(HashSet::new());
        entry.insert(step.before as char);
        set.entry(step.before as char).or_insert(HashSet::new());
    }

    let mut result = Vec::new();

    loop {
        let mut readies = Vec::new();
        for (after, befores) in set.iter() {
            if befores.len() == 0 {
                readies.push(*after);
            }
        }

        if readies.len() == 0 {
            break;
        }

        readies.sort();

        set.remove(&readies[0]);
        for (_key, val) in set.iter_mut() {
            val.remove(&readies[0]);
        }

        result.push(readies[0]);
    }

    result.iter().collect()
}

#[derive(Debug)]
struct Job {
    step: char,
    time_left: u64,
}

fn _b(steps: Vec<Step>, extra_time: u64, elves: u64) -> u64 {
    let mut set: HashMap<char, HashSet<char>> = HashMap::new();

    for step in steps.iter() {
        let entry = set.entry(step.after as char).or_insert(HashSet::new());
        entry.insert(step.before as char);
        set.entry(step.before as char).or_insert(HashSet::new());
    }

    let mut assigned: BTreeMap<u64, Option<Job>> = BTreeMap::new();
    for i in 0..elves {
        assigned.insert(i, None);
    }

    let mut total_time = 1u64;
    let mut unassigned = Vec::new();
    'outer: loop {
        unassigned.extend(
            set.drain_filter(|_after, befores| befores.len() == 0)
                .map(|(key, _val)| key),
        );

        for (_elf, elf_job) in assigned.iter_mut() {
            if elf_job.is_none() && unassigned.len() > 0 {
                let newjob = unassigned.remove(0);
                let step_time: u64 = extra_time + ((newjob as u64) - 64);
                *elf_job = Some(Job {
                    step: newjob,
                    time_left: step_time,
                });
            }
        }

        for (_elf, elf_job) in assigned.iter_mut() {
            if let Some(job) = elf_job {
                job.time_left -= 1;
                if job.time_left == 0 {
                    for (_after, val) in set.iter_mut() {
                        val.remove(&job.step);
                    }
                    *elf_job = None;
                    if unassigned.len() == 0 && set.len() == 0 {
                        break 'outer;
                    }
                }
            }
        }

        total_time += 1;
    }

    total_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            "CABDFE",
            _a(vec![
                "Step C must be finished before step A can begin.".to_string(),
                "Step C must be finished before step F can begin.".to_string(),
                "Step A must be finished before step B can begin.".to_string(),
                "Step A must be finished before step D can begin.".to_string(),
                "Step B must be finished before step E can begin.".to_string(),
                "Step D must be finished before step E can begin.".to_string(),
                "Step F must be finished before step E can begin.".to_string(),
            ]
            .into_iter()
            .map(|item| parse_step(item))
            .collect())
        );
    }

    #[test]
    fn a2() {
        assert_eq!("HEGMPOAWBFCDITVXYZRKUQNSLJ", _a(load_from_file()));
    }

    #[test]
    fn b() {
        assert_eq!(
            15,
            _b(
                vec![
                    "Step C must be finished before step A can begin.".to_string(),
                    "Step C must be finished before step F can begin.".to_string(),
                    "Step A must be finished before step B can begin.".to_string(),
                    "Step A must be finished before step D can begin.".to_string(),
                    "Step B must be finished before step E can begin.".to_string(),
                    "Step D must be finished before step E can begin.".to_string(),
                    "Step F must be finished before step E can begin.".to_string(),
                ]
                .into_iter()
                .map(|item| parse_step(item))
                .collect(),
                0,
                2
            )
        );
    }

    #[test]
    fn b2() {
        assert_eq!(1226, _b(load_from_file(), 60, 5));
    }
}
