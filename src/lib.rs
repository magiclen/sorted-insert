/*!
# Sorted Insert

This crate provides traits to insert elements to a sorted collection and keep the order.

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
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(deque_make_contiguous))]

extern crate alloc;

mod collections;

use core::cmp::Ordering;

#[doc(hidden)]
pub trait SortedInsertBasic<T> {
    #[doc(hidden)]
    fn insert_element(&mut self, index: usize, element: T);
}

pub trait SortedInsertBy<T>: SortedInsertBasic<T> {
    /// Insert elements to this sorted collection by a specific comparator and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_by<F: FnMut(&T, &T) -> bool>(&mut self, element: T, mut f: F) -> usize {
        let index = self.get_sorted_insert_index_by(|e| f(e, &element));

        self.insert_element(index, element);

        index
    }

    #[doc(hidden)]
    fn get_sorted_insert_index_by<F: FnMut(&T) -> bool>(&self, f: F) -> usize;
}

pub trait SortedInsertByKey<T>: SortedInsertBy<T> {
    /// Insert elements to this sorted collection in ascending order by a specific key and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_asc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: T,
        mut f: F,
    ) -> usize {
        self.sorted_insert_by(element, |e, element| f(e) <= f(&element))
    }

    /// Insert elements to this sorted collection in descending order by a specific key and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_desc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: T,
        mut f: F,
    ) -> usize {
        self.sorted_insert_by(element, |e, element| f(e) >= f(&element))
    }
}

pub trait SortedInsert<T: Ord>: SortedInsertByKey<T> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_asc(&mut self, element: T) -> usize {
        self.sorted_insert_asc_by_key(element, |element| element)
    }

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    fn sorted_insert_desc(&mut self, element: T) -> usize {
        self.sorted_insert_desc_by_key(element, |element| element)
    }
}

pub trait SortedInsertBinaryBy<T>: SortedInsertBy<T> {
    /// Insert elements to this sorted collection by a specific comparator and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    fn sorted_insert_binary_by<F: FnMut(&T, &T) -> Ordering>(
        &mut self,
        element: T,
        mut f: F,
    ) -> usize {
        let index = self.get_sorted_insert_index_binary_by(|e| f(e, &element));

        self.insert_element(index, element);

        index
    }

    #[doc(hidden)]
    fn get_sorted_insert_index_binary_by<F: FnMut(&T) -> Ordering>(&mut self, f: F) -> usize;
}

pub trait SortedInsertBinaryByKey<T>: SortedInsertBinaryBy<T> {
    /// Insert elements to this sorted collection in ascending order by a specific key and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_binary_asc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: T,
        mut f: F,
    ) -> usize {
        self.sorted_insert_binary_by(element, |e, element| f(e).cmp(f(element)))
    }

    /// Insert elements to this sorted collection in descending order by a specific key and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_binary_desc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: T,
        mut f: F,
    ) -> usize {
        self.sorted_insert_binary_by(element, |e, element| f(element).cmp(f(e)))
    }
}

pub trait SortedInsertBinary<T: Ord>: SortedInsertBinaryByKey<T> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_asc_binary(&mut self, element: T) -> usize {
        self.sorted_insert_binary_asc_by_key(element, |element| element)
    }

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_desc_binary(&mut self, element: T) -> usize {
        self.sorted_insert_binary_desc_by_key(element, |element| element)
    }
}
