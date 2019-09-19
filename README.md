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
                        change: [+0.9983% +1.7070% +2.4129%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)                                                                           3 (3.00%) low severe

trie lookup             time:   [1.6120 us 1.6521 us 1.7048 us]
                        change: [-36.625% -3.5055% +46.407%] (p = 0.88 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)                                                                         4 (4.00%) high mild
  9 (9.00%) high severe

old trie insert         time:   [902.92 ns 925.90 ns 956.43 ns]
                        change: [-26.627% -0.6815% +35.536%] (p = 0.96 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)                                                                         14 (14.00%) low mild
  1 (1.00%) high severe

old trie lookup         time:   [5.4246 us 5.6215 us 5.8838 us]
                        change: [-40.977% -1.7807% +62.650%] (p = 0.95 > 0.05)
                        No change in performance detected.
```