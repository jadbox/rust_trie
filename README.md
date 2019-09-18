# rust_trie

Rust trie experiments

## List of optimizations

* switched node hashmap to FnvHashMap which is far faster for small/char hash keys 
* added an (lifeguard) object pool to preallocate boxed Nodes to speed up insertion
* removed Vec<char> for String in lookup, because String.clone is faster than iter().collect()
* removed a few unneeded clones/allocs in lookup
* logic change: removed the "explored" check in lookup, as every node in the tree is unique and there shouldn't ever be a case where the same Node is added to the frontier as a dup.
