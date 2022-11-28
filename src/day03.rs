use std::fs::File;
use std::io::{BufRead, BufReader};

struct Claim {
    id: u64,
    x: u64,
    y: u64,
    w: u64,
    h: u64,
}

fn parse_claim(line: String) -> Claim {
    // Example: #1 @ 1,3: 4x4
    let parts: Vec<&str> = line.split(" ").collect();
    let xy: Vec<&str> = parts[2][..parts[2].len() - 1].split(",").collect();
    let wh: Vec<&str> = parts[3].split("x").collect();

    Claim {
        id: parts[0][1..].parse::<u64>().unwrap(),
        x: xy[0].parse::<u64>().unwrap(),
        y: xy[1].parse::<u64>().unwrap(),
        w: wh[0].parse::<u64>().unwrap(),
        h: wh[1].parse::<u64>().unwrap(),
    }
}

fn load_from_file() -> Vec<Claim> {
    let file = File::open("input/day03.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| parse_claim(line.unwrap()))
        .collect()
}

pub fn a() -> u64 {
    _a(load_from_file())
}

fn _a(claims: Vec<Claim>) -> u64 {
    let mut result = 0u64;
    let mut grid = vec![0u64; 1000_000];
    for claim in claims {
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                let idx = (y * 1000 + x) as usize;
                grid[idx] += 1;
                if grid[idx] == 2 {
                    result += 1;
                }
            }
        }
    }

    result
}

pub fn b() -> u64 {
    _b(load_from_file())
}

fn _b(claims: Vec<Claim>) -> u64 {
    let mut grid = vec![0u64; 1000_000];
    for claim in claims.iter() {
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                let idx = (y * 1000 + x) as usize;
                grid[idx] += 1;
            }
        }
    }
    'outer: for claim in claims.iter() {
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                if grid[(y * 1000 + x) as usize] != 1 {
                    continue 'outer
                }
            }
        }
        return claim.id
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            4,
            _a(vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
                .iter()
                .map(|line| parse_claim(line.to_string()))
                .collect())
        )
    }

    #[test]
    fn a2() {
        assert_eq!(103806, _a(load_from_file()))
    }

    #[test]
    fn b() {
        assert_eq!(
            3,
            _b(vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
                .iter()
                .map(|line| parse_claim(line.to_string()))
                .collect())
        )
    }

    #[test]
    fn b2() {
        assert_eq!(625, _b(load_from_file()))
    }
}
