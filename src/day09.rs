use std::{cell::RefCell, rc::Rc};

use crate::aoc::load_text;

pub fn a() -> usize {
    _a(load_text(9))
}

pub fn b() -> usize {
    _b(load_text(9))
}

type RcMarble = Rc<RefCell<Marble>>;

struct Marble {
    prev: Option<RcMarble>,
    next: Option<RcMarble>,
    value: usize,
}

impl Marble {
    fn prev(self: &Marble) -> RcMarble {
        self.prev.as_ref().cloned().unwrap()
    }

    fn next(self: &Marble) -> RcMarble {
        self.next.as_ref().cloned().unwrap()
    }

    fn remove(self: &mut Marble) {
        let prev = self.prev();
        let next = self.next();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
        self.next = None;
        self.prev = None;
    }
}

fn parse(line: &str) -> (usize, usize) {
    let parts: Vec<&str> = line.split(' ').collect();
    (parts[0].parse().unwrap(), parts[6].parse().unwrap())
}

fn _a(line: impl AsRef<str>) -> usize {
    _ab(line, 1)
}

fn _b(line: impl AsRef<str>) -> usize {
    _ab(line, 100)
}

fn _ab(line: impl AsRef<str>, mul: usize) -> usize {
    let (players, mut last_marble) = parse(line.as_ref());
    last_marble *= mul;
    let mut current = Rc::new(RefCell::new(Marble {
        prev: None,
        next: None,
        value: 0,
    }));
    current.borrow_mut().prev = Some(current.clone());
    current.borrow_mut().next = Some(current.clone());

    let mut player = 0;
    let mut scores = vec![0usize; players];

    for i in 1..=last_marble {
        if i % 23 == 0 {
            scores[player] += i;
            for _j in 0..7 {
                let next = current.borrow().prev();
                current = next;
            }
            let next = current.borrow().next();
            current.borrow_mut().remove();
            scores[player] += current.borrow().value;
            current = next;

            player = (player + 1) % players;
            continue;
        }
        let next = current.borrow().next();
        let next_next = next.borrow().next();

        let new_marble = Rc::new(RefCell::new(Marble {
            prev: Some(next.clone()),
            next: Some(next_next.clone()),
            value: i,
        }));
        next.borrow_mut().next = Some(new_marble.clone());
        next_next.borrow_mut().prev = Some(new_marble.clone());
        current = new_marble;

        player = (player + 1) % players;
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a1() {
        assert_eq!(8317, _a("10 players; last marble is worth 1618 points"));
    }

    #[test]
    fn a2() {
        assert_eq!(146373, _a("13 players; last marble is worth 7999 points"));
    }

    #[test]
    fn a3() {
        assert_eq!(2764, _a("17 players; last marble is worth 1104 points"));
    }

    #[test]
    fn a4() {
        assert_eq!(54718, _a("21 players; last marble is worth 6111 points"));
    }

    #[test]
    fn a5() {
        assert_eq!(37305, _a("30 players; last marble is worth 5807 points"));
    }

    #[test]
    fn a6() {
        assert_eq!(418237, _a(load_text(9)));
    }

    #[test]
    fn b1() {
        assert_eq!(3505711612, _b(load_text(9)));
    }
}
