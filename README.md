# rust_trie

Rust trie experiments

## List of optimizations

* switched node hashmap to FnvHashMap which is far faster for small/char hash keys 
* added an (lifeguard) object pool to preallocate boxed Nodes to speed up insertion
* removed Vec<char> for String in lookup, because String.clone is faster than iter().collect()
* removed a few unneeded clones/allocs in lookup
* logic change: removed the "explored" check in lookup, as every node in the tree is unique and there shouldn't ever be a case where the same Node is added to the frontier as a dup.
* changed String to &str on trie interfaces to reduce string copy
* removed boxes around Node from the Node's hashmap and pool

```bash
trie insert             time:   [243.75 ns 244.29 ns 244.93 ns]
Found 3 outliers among 100 measurements (3.00%)                                                                           3 (3.00%) low severe

trie lookup             time:   [1.6120 us 1.6521 us 1.7048 us]
Found 13 outliers among 100 measurements (13.00%)                                                                         4 (4.00%) high mild
  9 (9.00%) high severe

old trie insert         time:   [902.92 ns 925.90 ns 956.43 ns]
Found 15 outliers among 100 measurements (15.00%)                                                                         14 (14.00%) low mild
  1 (1.00%) high severe

old trie lookup         time:   [5.4246 us 5.6215 us 5.8838 us]
```