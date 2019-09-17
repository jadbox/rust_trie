use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use uuid::Uuid;
use std::time::{Duration, Instant};
extern crate lifeguard;
use lifeguard::*;

// const xid: Uuid = Uuid::new_v4();

#[derive(Eq, Debug)]
struct Node {
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

impl lifeguard::Recycleable for Box<Node> {
    fn new() -> Self {
        return Box::new(Node::new('!'));
    }
    fn reset(&mut self){
        return;
    }
}

impl lifeguard::InitializeWith<char> for Box<Node> {
    fn initialize_with(&mut self, source: char) {
        self.val = source;
    }
}

impl Node {
    fn new(c: char) -> Node {
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

    fn insert(&mut self, s: String) {
        let mut next = self;
        for c in s.chars() {
            next = next
                .children
                .entry(c)
                .or_insert_with(|| Box::new(Node::new(c)));
        }

        next.set_terminal()
    }

    fn insert_bypool(&mut self, s: String, pool: &mut Pool<Box<Node>>) {
        let mut next = self;
        for c in s.chars() {
            next = next
                .children
                .entry(c)
                .or_insert_with(|| pool.new_from(c).detach())
        }

        next.set_terminal()
    }

    fn lookup(&self, s: String) -> (bool, Vec<String>) {
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

        // let mut explored: HashSet<Uuid> = HashSet::new();
        let mut frontier: Vec<(&Box<Node>, String)> = Vec::new();
        let mut path: String = s; // .clone(); // No need to .clone(), already own it by copy

        while true {
            // terminal node - should also add check for null char (we could have brom and brom, but it would not)
            if next.children.len() == 0 || next.is_terminal {
                suggestions.push(path.clone());
            }

            for (_, v) in &next.children {
                // if !explored.contains(&v.id) {
                    frontier.push( (v, path.clone() ))
                // }
            }

            // explored.insert(next.id);

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

fn bench() {
    // https://raw.githubusercontent.com/dwyl/english-words/master/words.txt
    let f = File::open("words.txt").unwrap();
    let f = BufReader::new(f);


    let mut words: Vec<String> = f.lines().map(|x| x.unwrap()).collect();

    let mut t0;
    let mut trie = Node::new('\x00');
    let mut pool:Pool<Box<Node>> = pool().with(StartingSize(words.len())).build();

    let mut inserts = Vec::new();
    let mut lookups = Vec::new();

    for w in words.iter() {
        t0 = Instant::now();
        trie.insert_bypool(w.clone(), &mut pool);
        inserts.push(t0.elapsed().as_nanos());
    }

    let mut res; 
    for w in words.iter() {
        t0 = Instant::now();
        res = trie.lookup(w.clone());
        lookups.push(t0.elapsed().as_nanos());
        assert!(res.0);
    }

    inserts.sort();
    lookups.sort();

    println!("running benchmark tests with {} items", inserts.len());
    println!("trie:  median insert {}, median lookup {}", inserts[inserts.len()/2], lookups[lookups.len()/2]);
    


    // let mut v = Vec::new();
    // let mut vinserts = Vec::new();
    // let mut vlookups = Vec::new();


    // for w in words.iter() {
    //     t0 = Instant::now();
    //     v.push(w.clone());
    //     vinserts.push(t0.elapsed().as_nanos());
    // }

    // let mut b = true;
    // for w in words.iter() {
    //     t0 = Instant::now();
    //     if v.contains(w) {
    //         b = true;
    //     }
    //     b = false;

    //     vlookups.push(t0.elapsed().as_nanos());

    // }

    
    // println!("vector:  median insert {}, median lookup {}", vinserts[inserts.len()/2], vlookups[lookups.len()/2]);

    let mut h = HashSet::new();
    let mut hinserts = Vec::new();
    let mut hlookups = Vec::new();


    for w in words.iter() {
        t0 = Instant::now();
        h.insert(w.clone());
        hinserts.push(t0.elapsed().as_nanos());
    }

    let mut b = true;
    for w in words.iter() {
        t0 = Instant::now();
        h.contains(w);
        hlookups.push(t0.elapsed().as_nanos());

    }

    hinserts.sort();
    hlookups.sort();

    // println!("{}", b);
    println!("hashset: median insert {}, median lookup {}", hinserts[inserts.len()/2], hlookups[lookups.len()/2]);







}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a() {
        let mut trie = Node::new('\x00');
        trie.insert("brom".to_string());
        let (c, sugs) = trie.lookup("cat".to_string());
        assert!(c == false);
        assert!(sugs.len() == 0);
    }

    #[test]
    fn b() {
        let mut trie = Node::new('\x00');
        trie.insert("brom".to_string());
        let (c, sugs) = trie.lookup("brom".to_string());
        assert!(c);
    }

    #[test]
    fn c() {
        let mut trie = Node::new('\x00');
        
        let tests = vec!["brom", "broom", "brook", "brooks brothers"];
        
        for t in tests.iter() {
            trie.insert(t.to_string());
        }

        
        let (c, sugs) = trie.lookup("bro".to_string());
        assert!(!c); //the trie does not contain 'bro' 
        for t in tests.iter() {
            println!("testing {}", t);
            assert!(sugs.contains(&t.to_string()));
            println!("\t{} ok!", t);
        }

        let (c, sugs) = trie.lookup("brom".to_string());
        assert!(c);
        assert!(sugs.len() == 1);
    

    }

}

fn main() {
    bench();
}
