use std::{fs::File, io::Read};

fn load_from_file() -> Vec<u64> {
    let mut file = File::open("input/day08.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    parse(line.strip_suffix('\n').unwrap())
}

fn parse(line: &str) -> Vec<u64> {
    line.split(' ').map(|item| item.parse().unwrap_or_else(|_| panic!("VALUE: '{item}'"))).collect()
}

pub fn a() -> u64 {
    _a(load_from_file())
}

pub fn b() -> u64 {
    _b(load_from_file())
}

struct Node {
    size: usize,
    metadata: u64,
    value: u64,
}

fn scan_node(nmbrs: &Vec<u64>, pos: usize) -> Node {
    let nr_child_nodes = nmbrs[pos];
    let nr_metadatas = nmbrs[pos+1];
    let mut size = 2usize;
    let mut metadata_total = 0u64;
    let mut child_nodes = Vec::new();
    let mut value = 0u64;

    for _i in 0..nr_child_nodes {
        let node = scan_node(nmbrs, pos + size);
        size += node.size;
        metadata_total += node.metadata;
        child_nodes.push(node);
    }
    for _i in 0..nr_metadatas {
        let metadata = nmbrs[pos+size];
        metadata_total += metadata;
        if nr_child_nodes == 0 {
            value += metadata;
        } else if metadata != 0 {
            let i = (metadata-1) as usize;
            if i < child_nodes.len() {
                value += child_nodes[i].value;
            }
        }
        size += 1;
    }

    Node{size, metadata: metadata_total, value}
}

fn _a(nmbrs: Vec<u64>) -> u64 {
    let node = scan_node(&nmbrs, 0);
    node.metadata
}

fn _b(nmbrs: Vec<u64>) -> u64 {
    let node = scan_node(&nmbrs, 0);
    node.value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(138, _a(parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")));
    }

    #[test]
    fn a2() {
        assert_eq!(49180, _a(load_from_file()));
    }

    #[test]
    fn b() {
        assert_eq!(66, _b(parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")));
    }

    #[test]
    fn b2() {
        assert_eq!(20611, _b(load_from_file()));
    }
}
