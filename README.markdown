### editdistancewf

A simple crate for finding the
[edit-distance](https://en.wikipedia.org/wiki/Edit_distance) between
two vectors, using the
[Wagner-Fischer](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm)
algorithm.

#### install

Through crates, as you would usually:

```toml
[dependencies]
edit-distance = "0.1.0"
```

#### usage

There is a single function in the `editdistancewf` namespace,
`distance`, that accepts two iterators for a type `T : Eq`. The
iterators are consumed by the comparison.

```rust
extern create editdistancewf as wf;

wf::distance("foo".chars(), "bar".chars())
    // => 3 : usize
```

##### license

MIT