use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Guard {
    sleep: u64,
    minutes: [u64; 60],
}

fn load_from_file() -> Vec<String> {
    let file = File::open("input/day04.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|item| item.unwrap()).collect()
}

pub fn a() -> u64 {
    _a(load_from_file())
}

pub fn b() -> u64 {
    _b(load_from_file())
}

fn _a(mut events: Vec<String>) -> u64 {
    events.sort_unstable();

    let mut hist: HashMap<u64, Guard> = HashMap::new();

    let mut cur_id = 0u64;
    let mut start_sleep = 0u64;
    let mut max_sleep = 0u64;
    let mut max_id = 0u64;

    for event in events.iter() {
        let parts: Vec<&str> = event.split(" ").collect();

        if event.contains("Guard") {
            cur_id = parts[3][1..].parse::<u64>().unwrap();
            hist.entry(cur_id).or_insert(Guard {
                sleep: 0,
                minutes: [0; 60],
            });
        } else if event.contains("falls") {
            let subparts: Vec<&str> = parts[1][..parts[1].len() - 1].split(":").collect();
            let minute = subparts[1].parse::<u64>().unwrap();
            start_sleep = minute;
        } else {
            let subparts: Vec<&str> = parts[1][..parts[1].len() - 1].split(":").collect();
            let end_sleep = subparts[1].parse::<u64>().unwrap();
            hist.entry(cur_id).and_modify(|v| {
                (*v).sleep += end_sleep - start_sleep;
                if (*v).sleep > max_sleep {
                    max_sleep = (*v).sleep;
                    max_id = cur_id;
                }
                for i in start_sleep..end_sleep {
                    (*v).minutes[i as usize] += 1;
                }
            });
        }
    }

    let guard = hist.get(&max_id).unwrap();
    let mut max_count = 0u64;
    let mut max_min = 0u64;
    for (min, count) in guard.minutes.iter().enumerate() {
        if *count > max_count {
            max_count = *count;
            max_min = min as u64;
        }
    }
    max_min * max_id
}

fn _b(mut events: Vec<String>) -> u64 {
    events.sort_unstable();

    let mut hist: HashMap<u64, Guard> = HashMap::new();

    let mut cur_id = 0u64;
    let mut start_sleep = 0u64;
    let mut max_id = 0u64;
    let mut max_min = 0u64;
    let mut max_sleep_at_min = 0u64;

    for event in events.iter() {
        let parts: Vec<&str> = event.split(" ").collect();

        if event.contains("Guard") {
            cur_id = parts[3][1..].parse::<u64>().unwrap();
            hist.entry(cur_id).or_insert(Guard {
                sleep: 0,
                minutes: [0; 60],
            });
        } else if event.contains("falls") {
            let subparts: Vec<&str> = parts[1][..parts[1].len() - 1].split(":").collect();
            let minute = subparts[1].parse::<u64>().unwrap();
            start_sleep = minute;
        } else {
            let subparts: Vec<&str> = parts[1][..parts[1].len() - 1].split(":").collect();
            let end_sleep = subparts[1].parse::<u64>().unwrap();
            hist.entry(cur_id).and_modify(|v| {
                (*v).sleep += end_sleep - start_sleep;
                for i in start_sleep..end_sleep {
                    (*v).minutes[i as usize] += 1;
                    if (*v).minutes[i as usize] > max_sleep_at_min {
                        max_sleep_at_min = (*v).minutes[i as usize];
                        max_min = i;
                        max_id = cur_id;
                    }
                }
            });
        }
    }

    max_min * max_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let result = _a(vec![
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up".to_string(),
        ]);
        assert_eq!(240, result);
    }

    #[test]
    fn b() {
        let result = _b(vec![
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up".to_string(),
        ]);
        assert_eq!(4455, result);
    }

    #[test]
    fn a2() {
        assert_eq!(73646, _a(load_from_file()));
    }

    #[test]
    fn b2() {
        assert_eq!(4727, _b(load_from_file()));
    }
}
