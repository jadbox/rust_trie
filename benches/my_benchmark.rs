#[macro_use]
extern crate trie;
use trie::trie::Node;

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
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(2));
    targets = criterion_benchmark
}
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    // https://raw.githubusercontent.com/dwyl/english-words/master/words.txt
    let f = File::open("words.txt").unwrap();
    let f = BufReader::new(f);

    let words2: Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    let words = words2[0..60000].to_vec();
    
    // words = [words.clone(), words].concat(); // double the size

    let mut trie = Node::new('\x00');
    let mut pool: Pool<Box<Node>> = pool().with(StartingSize(words.len() * 2)).build();

    // let mut inserts = Vec::new();
    // let mut lookups = Vec::new();

    // let mut group = c.benchmark_group("small");
    // group.sample_size(1);
    // c.sample_size(2);
    c.bench_function("trie insert", |b| {
        b.iter_batched_ref(|| words.clone(), |words| {
            let mut trie = Node::new('\x00');
            for w in words.iter() {
                // t0 = Instant::now();
                trie.insert_bypool(&w, &mut pool);
                // inserts.push(t0.elapsed().as_nanos());
            }
        }, BatchSize::LargeInput)
    });
    // group.finish();

    for w in words.iter() {
        trie.insert(&w);
    }

    // let mut res;
    c.bench_function("trie lookup", |b| {
        b.iter_batched_ref(|| words.clone() , |words| {
            for w in words.iter() {
                black_box(trie.lookup(&w, None, None));
            }
        }, BatchSize::LargeInput)
    });

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

    c.bench_function("hash insert", |b| {
        b.iter_batched_ref(|| words.clone() , |words| {
            let mut h = HashSet::new();
            for w in words.iter() {
                 h.insert(w);
            }
        }, BatchSize::LargeInput)
    });

    let mut h = HashSet::new();

    for w in words.iter() {
        h.insert(w);
    }

    c.bench_function("hash lookup", |b| {
        b.iter_batched_ref(|| words.clone() , |words| {
            for w in words.iter() {
                black_box(h.get(w));
            }
        }, BatchSize::LargeInput)
    });
}
