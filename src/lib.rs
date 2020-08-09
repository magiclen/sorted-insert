/*!
# Sorted Insert

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

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.sorted-insert]
version = "*"
default-features = false
```
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(deque_make_contiguous))]

extern crate alloc;

mod collections;

pub trait SortedInsert<T: Ord> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    fn sorted_insert_asc(&mut self, element: T) -> usize;

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    fn sorted_insert_desc(&mut self, element: T) -> usize;
}

pub trait SortedInsertBinary<T: Ord> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    fn sorted_insert_asc_binary(&mut self, element: T) -> usize;

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    fn sorted_insert_desc_binary(&mut self, element: T) -> usize;
}
