#[macro_use]
extern crate trie;
use trie::trie::Node;
use trie::trie_old::Node as OldNode;

use fnv::FnvHashMap;
use lifeguard::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::{Duration, Instant};
use uuid::Uuid;

// use trie::*;
// #[path = "../src/main.rs"]
// mod trie;

extern crate criterion;
use criterion::*;

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(2)).warm_up_time(Duration::from_secs(1));
    targets = criterion_benchmark
}
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    // https://raw.githubusercontent.com/dwyl/english-words/master/words.txt
    let f = File::open("words.txt").unwrap();
    let f = BufReader::new(f);

    let words: Vec<String> = f.lines().map(|x| x.unwrap()).collect();

    let mut pool: Pool<Box<Node>> = pool().with(StartingSize(words.len() * 2)).build();

    let mut trie = Node::new('\x00');
    c.bench_function("trie insert", |b| {
        let mut i = 0;
        trie = Node::new('\x00');
    
        b.iter(|| {
            trie.insert_bypool(&words[i], &mut pool);
            i = i + 1;
            if i == words.len() {
                i = 0;
            }
        })
    });

    trie = Node::new('\x00');
    for w in words.iter() {
        trie.insert(&w);
    }

    c.bench_function("trie lookup", |b| {
        let mut i = 0;
        b.iter(|| {
            black_box(trie.lookup(&words[i], None, None));
            i = i + 1;
            if i == words.len() {
                i = 0;
            }
        })
    });

    
    let mut trie2 = OldNode::new('\x00');
    c.bench_function("old trie insert", |b| {
        trie2 = OldNode::new('\x00');
        let mut i = 0;
    
        b.iter(|| {
            trie2.insert(words[i].clone());
            i = i + 1;
            if i == words.len() {
                i = 0;
            }
        })
    });

    trie2 = OldNode::new('\x00');
    for w in words.iter() {
        trie2.insert(w.clone());
    }

    // let mut res;
    c.bench_function("old trie lookup", |b| {
        let mut i = 0;
        b.iter(|| {
            black_box(trie2.lookup(words[i].clone()));
            i = i + 1;
            if i == words.len() {
                i = 0;
            }
        })
    });
}
