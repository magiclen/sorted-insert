Sorted Insert
====================

[![Build Status](https://travis-ci.org/magiclen/sorted-insert.svg?branch=master)](https://travis-ci.org/magiclen/sorted-insert)

This crate provides traits to insert elements to a sorted collection and remain the order.

## Examples

```rust
extern crate sorted_insert;

use sorted_insert::SortedInsert;

let mut v = vec![1, 5];

v.sorted_insert_asc(2);

assert_eq!([1, 2, 5], v.as_slice());
```

```rust
extern crate sorted_insert;

use sorted_insert::SortedInsertBinary;

let mut v = vec![5, 1];

v.sorted_insert_desc_binary(2);

assert_eq!([5, 2, 1], v.as_slice());
```

```rust
extern crate sorted_insert;

use sorted_insert::SortedInsertByKey;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct A(i32, i32);

let mut v = vec![A(1, 10), A(2, 20)];

v.sorted_insert_asc_by_key(A(1, 15), |e| &e.1);

assert_eq!([A(1, 10), A(1, 15), A(2, 20)], v.as_slice());
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.sorted-insert]
version = "*"
default-features = false
```

## Crates.io

https://crates.io/crates/sorted-insert

## Documentation

https://docs.rs/sorted-insert

## License

[MIT](LICENSE)