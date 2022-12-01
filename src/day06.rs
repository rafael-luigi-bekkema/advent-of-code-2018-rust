use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_from_file() -> Vec<Point> {
    let file = File::open("input/day06.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| parse_coord(line.unwrap()))
        .collect()
}

struct Point {
    x: u64,
    y: u64,
}

fn parse_coord(line: String) -> Point {
    let (x, y) = line.split_once(", ").unwrap();
    Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

pub fn a() -> u64 {
    _a(load_from_file())
}

pub fn b() -> u64 {
    _b(load_from_file(), 10_000)
}

fn _a(points: Vec<Point>) -> u64 {
    let mut areas = HashMap::new();
    let (mut minx, mut maxx, mut miny, mut maxy) = (0u64, 0u64, 0u64, 0u64);
    for (i, point) in points.iter().enumerate() {
        if point.x - 1 < minx {
            minx = point.x - 1;
        }
        if i == 0 || point.x + 1 > maxx {
            maxx = point.x + 1;
        }
        if point.y - 1 < miny {
            miny = point.y - 1
        }
        if i == 0 || point.y + 1 > maxy {
            maxy = point.y + 1;
        }
        areas.insert(i, 0);
    }
    for y in miny..=maxy {
        'loopx: for x in minx..=maxx {
            let mut mindist = 0u64;
            let mut minpoint = 0usize;
            let mut mincount = 0usize;
            for (i, point) in points.iter().enumerate() {
                let dist = point.x.abs_diff(x) + point.y.abs_diff(y);
                if i > 0 && dist == mindist {
                    mincount += 1;
                }
                if i == 0 || dist < mindist {
                    mindist = dist;
                    minpoint = i;
                    mincount = 1;
                }
            }

            if mincount > 1 {
                continue 'loopx;
            }

            if x == minx || x == maxx || y == miny || y == maxy {
                areas.remove(&minpoint);
                continue;
            }
            areas.entry(minpoint).and_modify(|item| *item += 1);
        }
    }

    let mut maxcount = 0u64;
    for (_i, area) in areas.iter() {
        if *area > maxcount {
            maxcount = *area;
        }
    }
    maxcount
}

fn _b(points: Vec<Point>, maxtotal: u64) -> u64 {
    let (mut minx, mut maxx, mut miny, mut maxy) = (0u64, 0u64, 0u64, 0u64);
    for (i, point) in points.iter().enumerate() {
        if point.x - 1 < minx {
            minx = point.x - 1;
        }
        if i == 0 || point.x + 1 > maxx {
            maxx = point.x + 1;
        }
        if point.y - 1 < miny {
            miny = point.y - 1
        }
        if i == 0 || point.y + 1 > maxy {
            maxy = point.y + 1;
        }
        // areas.insert(i, 0);
    }
    let mut size = 0u64;
    for y in miny..=maxy {
        'loopx: for x in minx..=maxx {
            let mut totaldist = 0u64;
            for point in points.iter() {
                let dist = point.x.abs_diff(x) + point.y.abs_diff(y);
                totaldist += dist;
            }
            if totaldist >= maxtotal {
                continue 'loopx
            }
            if totaldist < maxtotal {
                size += 1;
            }
        }
    }
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            17,
            _a(vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 6 },
                Point { x: 8, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 5, y: 5 },
                Point { x: 8, y: 9 },
            ])
        )
    }

    #[test]
    fn a2() {
        assert_eq!(3449, _a(load_from_file()));
    }

    #[test]
    fn b() {
        assert_eq!(
            16,
            _b(
                vec![
                    Point { x: 1, y: 1 },
                    Point { x: 1, y: 6 },
                    Point { x: 8, y: 3 },
                    Point { x: 3, y: 4 },
                    Point { x: 5, y: 5 },
                    Point { x: 8, y: 9 },
                ],
                32
            )
        )
    }

    #[test]
    fn b2() {
        assert_eq!(44868, _b(load_from_file(), 10_000));
    }
}
