use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::{Duration, Instant};
use uuid::Uuid;

use fnv::FnvHashMap;
use lifeguard::*;

#[derive(Debug)]
pub struct Node {
    val: char,
    // id: Uuid,
    children: FnvHashMap<char, Box<Node>>,
    is_terminal: bool,
}

impl lifeguard::Recycleable for Box<Node> {
    fn new() -> Self {
        return Box::new(Node::new('!'));
    }
    fn reset(&mut self) {
        return;
    }
}

impl lifeguard::InitializeWith<char> for Box<Node> {
    fn initialize_with(&mut self, source: char) {
        self.val = source;
    }
}

impl Node {
    pub fn new(c: char) -> Node {
        // let id = Uuid::new_v4();
        // println!("new Node with char {} and id {}", c, id);
        Node {
            val: c,
            // id: id,
            children: FnvHashMap::default(),
            is_terminal: false,
        }
    }

    fn set_terminal(&mut self) {
        self.is_terminal = true;
    }

    pub fn insert(&mut self, s: &str) {
        let mut next = self;
        for c in s.chars() {
            next = next
                .children
                .entry(c)
                .or_insert_with(|| Box::new(Node::new(c)));
        }

        next.set_terminal()
    }

    pub fn insert_bypool(&mut self, s: &str, pool: &mut Pool<Box<Node>>) {
        let mut next = self;
        for c in s.chars() {
            next = next
                .children
                .entry(c)
                .or_insert_with(|| pool.new_from(c).detach())
        }

        next.set_terminal()
    }

    pub fn lookup(
        &self,
        s: &str,
        length: Option<usize>,
        limit: Option<usize>,
    ) -> (bool, Vec<String>) {
        // returns (true/false if it contains), list of autocompletes
        let mut next = self;
        let mut contains = false;
        let mut suggestions = Vec::new();

        let _length = length.unwrap_or(std::usize::MAX);
        let _limit = limit.unwrap_or(std::usize::MAX);

        for c in s.chars() {
            match next.children.get(&c) {
                Some(node) => {
                    next = node;
                }
                _ => {
                    contains = false;
                    return (contains, suggestions);
                }
            }
        }

        if next.is_terminal {
            contains = true;
        }

        // let mut explored: HashSet<Uuid> = HashSet::new();
        let mut frontier: Vec<(&Box<Node>, String)> = Vec::new();
        // let pathi = s.to_string();
        let mut path = s.to_string(); // No need to .clone(), already own it by copy

        while true {
            if suggestions.len() >= _limit {
                break;
            }
            // terminal node - should also add check for null char
            if next.children.len() == 0 || next.is_terminal {
                suggestions.push(path.clone());
            }

            if path.len() < _length {
                for (_, v) in &next.children {
                    let mut c = path.clone();
                    c.push(v.val);
                    frontier.push((v, c))
                }
            }

            match frontier.pop() {
                Some(t) => {
                    next = t.0;
                    path = t.1;
                }
                _ => break,
            }
        }

        (contains, suggestions)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a() {
        let mut trie = Node::new('\x00');
        trie.insert("brom");
        let (c, sugs) = trie.lookup("cat", None, None);
        assert!(c == false);
        assert!(sugs.len() == 0);
    }

    #[test]
    fn b() {
        let mut trie = Node::new('\x00');
        trie.insert("brom");
        let (c, sugs) = trie.lookup("brom", None, None);
        assert!(c);
    }

    #[test]
    fn c() {
        let mut trie = Node::new('\x00');
        let tests = vec!["brom", "broom", "brook", "brooks brothers"];
        for t in tests.iter() {
            trie.insert(t);
        }

        let (c, sugs) = trie.lookup("bro", None, None);
        // println!("testing {:?}", sugs);

        assert!(!c); //the trie does not contain 'bro'
        for t in tests.iter() {
            println!("testing {}", t);
            assert!(sugs.contains(&t.to_string()));
            assert!(sugs.len() == tests.len());
            println!("\t{} ok!", t);
        }

        let (c, sugs) = trie.lookup("brom", None, None);
        assert!(c);
        assert!(sugs.len() == 1);

        // Check limit
        let (c, sugs) = trie.lookup("bro", None, Some(2));
        assert!(sugs.len() == 2);

        // Check depth
        let (c, sugs) = trie.lookup("bro", Some(4), None);
        println!("testing {:?}", sugs);
        assert!(sugs.len() == 1);
    }

    #[test]
    fn d() {
        println!("aa");
        let mut trie = Node::new('\x00');
        let tests = vec!["brom", "broom", "brook", "brooks brothers"];
        for t in tests.iter() {
            trie.insert(t);
        }

        // Check limit
        let (c, sugs) = trie.lookup("bro", None, Some(2));
        println!("testing bro limit 2 {:?}", sugs);
        assert!(sugs.len() == 2);

        // Check depth
        let (c, sugs) = trie.lookup("bro", Some(4), None);
        println!("testing bro length 4 {:?}", sugs);
        assert!(sugs.len() == 1);
    }
}

fn main() {
    // bench();
}
