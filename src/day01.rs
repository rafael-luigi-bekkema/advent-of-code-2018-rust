use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn a() -> i64 {
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file);

    let nums: Vec<i64> = reader
        .lines()
        .map(|item| item.unwrap().parse::<i64>().unwrap())
        .collect();
    _a(nums)
}

fn _a(nums: Vec<i64>) -> i64 {
    let mut val = 0i64;
    for num in nums {
        val += num;
    }
    val
}

pub fn b() -> i64 {
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file);

    let nums: Vec<i64> = reader
        .lines()
        .map(|item| item.unwrap().parse::<i64>().unwrap())
        .collect();

    _b(nums)
}

fn _b(nums: Vec<i64>) -> i64 {
    let mut hist = HashMap::new();
    let mut value = 0i64;
    'outer: loop {
        for num in nums.iter() {
            value += num;

            if hist.contains_key(&value) {
                break 'outer;
            }

            hist.insert(value, true);
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let result = _a(vec![1, -2, 3, 1]);
        assert_eq!(3, result);
    }

    #[test]
    fn b() {
        let result = _b(vec![1, -2, 3, 1]);
        assert_eq!(2, result);
    }
}
