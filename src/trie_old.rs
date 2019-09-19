
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{self, BufReader};
use uuid::Uuid;

#[derive(Eq, Debug)]
pub struct Node {
    val: char,
    id: Uuid,
    children: HashMap<char, Box<Node>>,
    is_terminal: bool,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
    }
}

impl Node {
    pub fn new(c: char) -> Node {
        let id = Uuid::new_v4();
        // println!("new Node with char {} and id {}", c, id);
        Node {
            val: c,
            id: id,
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    fn set_terminal(&mut self) {
        self.is_terminal = true;
    }

    pub fn insert(&mut self, s: String) {
        let mut next = self;
        for c in s.chars() {
            next = next
                .children
                .entry(c)
                .or_insert_with(|| Box::new(Node::new(c)));
        }

        next.set_terminal()
    }

    pub fn lookup(&self, s: String) -> (bool, Vec<String>) {
        // returns (true/false if it contains), list of autocompletes
        let mut next = self;
        let mut contains = false;
        let mut suggestions = Vec::new();

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

        let mut explored: HashSet<Uuid> = HashSet::new();
        let mut frontier: Vec<(&Box<Node>, Vec<char>)> = Vec::new();
        let mut path: Vec<char> = s.chars().collect();

        while true {
            // terminal node - should also add check for null char (we could have brom and brom, but it would not)
            if next.children.len() == 0 || next.is_terminal {
                suggestions.push(path.iter().collect());
            }

            for (k, v) in &next.children {
                if !explored.contains(&v.id) {
                    frontier.push((v, path.clone()));
                }
            }

            explored.insert(next.id);

            match frontier.pop() {
                Some(t) => {
                    next = t.0;
                    path = t.1;
                }
                _ => break,
            }

            path.push(next.val);
        }

        (contains, suggestions)
    }
}

fn main() {
    // bench();
}
