use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn load_from_file() -> Vec<u8> {
    let mut file = File::open("input/day05.txt").unwrap();

    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();

    line.strip_suffix('\n').unwrap().bytes().collect()
}

pub fn a() -> u64 {
    _a(load_from_file())
}

pub fn b() -> u64 {
    _b(load_from_file())
}

fn react(mut pol: Vec<u8>) -> u64 {
    let mut i = 0usize;
    while i < (pol.len() - 1) {
        if pol[i].abs_diff(pol[i + 1]) == 32 {
            pol.remove(i + 1);
            pol.remove(i);
            if i > 0 {
                i -= 1;
            }
            continue;
        }
        i += 1
    }

    pol.len() as u64
}

fn _a(pol: Vec<u8>) -> u64 {
    react(pol)
}

fn _b(pol: Vec<u8>) -> u64 {
    let mut set = HashSet::new();
    // Collect lower case items
    for item in pol.iter() {
        set.insert(match *item {
            0..=96 => *item + 32, // A .. Z
            _ => *item,
        });
    }

    let mut minlen = 0u64;

    for (i, item) in set.iter().enumerate() {
        let pol2 = pol
            .clone()
            .into_iter()
            .filter(|subitem| !(*item == *subitem || *item == *subitem + 32))
            .collect();

        let val = react(pol2);
        if i == 0 || val < minlen {
            minlen = val
        }
    }

    minlen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(10, _a("dabAcCaCBAcCcaDA".bytes().collect()))
    }

    #[test]
    fn a2() {
        assert_eq!(10368, _a(load_from_file()))
    }

    #[test]
    fn b() {
        assert_eq!(4, _b("dabAcCaCBAcCcaDA".bytes().collect()))
    }

    #[test]
    fn b2() {
        assert_eq!(4122, _b(load_from_file()))
    }
}
