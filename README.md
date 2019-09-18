# rust_trie

Rust trie experiments

## List of optimizations

* switched node hashmap to FnvHashMap which is far faster for small/char hash keys 
* added an (lifeguard) object pool to preallocate boxed Nodes to speed up insertion
* removed Vec<char> for String in lookup, because String.clone is faster than iter().collect()
* removed a few unneeded clones/allocs in lookup