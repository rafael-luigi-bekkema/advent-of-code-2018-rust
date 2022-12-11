


use crate::aoc::load_lines;

pub fn a() -> i64 {
    _a(load_lines(2))
}

pub fn b() -> String {
    _b(load_lines(2))
}

fn _a(words: Vec<String>) -> i64 {
    let mut num2 = 0i64;
    let mut num3 = 0i64;
    for word in words.iter() {
        let mut chars = [0i64; 26];
        for chr in word.bytes() {
            chars[usize::from(chr - b'a')] += 1;
        }
        let mut has2 = false;
        let mut has3 = false;
        for count in chars {
            if count == 2 {
                has2 = true;
            }
            if count == 3 {
                has3 = true;
            }
        }
        if has2 {
            num2 += 1
        }
        if has3 {
            num3 += 1
        }
    }

    num2 * num3
}

fn _b(words: Vec<String>) -> String {
    let mut result: String = String::new();
    let mut diffi = 0;

    'outer: for word in words.iter() {
        let w1: Vec<u8> = word.bytes().collect();
        for word2 in words.iter() {
            let mut diffs = 0u64;
            let w2: Vec<u8> = word2.bytes().collect();

            for i in 0..word.len() {
                if w1[i] != w2[i] {
                    diffs += 1;
                    diffi = i;
                }
            }
            if diffs == 1 {
                result = format!(
                    "{}{}",
                    &word[..diffi],
                    &word2[diffi + 1..]
                );
                break 'outer;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let result = _a(vec![
            "abcdef".into(),
            "bababc".into(),
            "abbcde".into(),
            "abcccd".into(),
            "aabcdd".into(),
            "abcdee".into(),
            "ababab".into(),
        ]);
        assert_eq!(12, result);
    }

    #[test]
    fn b() {
        let result = _b(vec![
            "abcde".into(),
            "fghij".into(),
            "klmno".into(),
            "pqrst".into(),
            "fguij".into(),
            "axcye".into(),
            "wvxyz".into(),
        ]);
        assert_eq!("fgij", result);
    }

    #[test]
    fn b2() {
        assert_eq!("krdmtuqjgwfoevnaboxglzjph", _b(load_lines(2)))
    }
}
