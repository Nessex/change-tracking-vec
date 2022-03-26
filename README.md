# change-tracking-vec

This is a rust crate providing a wrapper around a `Vec`, which keeps a counter of every time it has been modified.

Modification is considered to be any time you call a function requiring a mutable reference to the `Vec`.

## Usage

```
let ct_vec = ChangeTrackingVec::new();
let ct_vec_2 = ChangeTrackingVec::with_capacity(10);
// etc.
```

Check the revision counter with `revision() -> usize`:

```
// NOTE: This counter will wrap!
ct_vec.revision()
```

Check if the vec has changed since the last call to `changed() -> bool`:

```rust
ct_vec.changed()
```

## Limitations

 - Not Allocator aware
 - No replacement for `vec![]`
 - No unstable functions implemented

## State

Proof-of-concept, no docs and few tests. Not published to crates.io.