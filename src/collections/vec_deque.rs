use core::cmp::Ordering;
use std::collections::VecDeque;

use crate::{
    SortedInsert, SortedInsertBasic, SortedInsertBinary, SortedInsertBinaryBy,
    SortedInsertBinaryByKey, SortedInsertBy, SortedInsertByKey,
};

impl<T> SortedInsertBasic<T> for VecDeque<T> {
    #[inline]
    fn insert_element(&mut self, index: usize, element: T) {
        self.insert(index, element);
    }
}

impl<T> SortedInsertBy<T> for VecDeque<T> {
    #[inline]
    fn get_sorted_insert_index_by<F: FnMut(&T) -> bool>(&self, f: F) -> usize {
        match self.iter().rposition(f) {
            Some(i) => i + 1,
            None => 0,
        }
    }
}

impl<T> SortedInsertByKey<T> for VecDeque<T> {}

impl<T: Ord> SortedInsert<T> for VecDeque<T> {}

impl<T> SortedInsertBinaryBy<T> for VecDeque<T> {
    #[inline]
    fn get_sorted_insert_index_binary_by<F: FnMut(&T) -> Ordering>(&mut self, f: F) -> usize {
        match self.make_contiguous().binary_search_by(f) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }
}

impl<T> SortedInsertBinaryByKey<T> for VecDeque<T> {}

impl<T: Ord> SortedInsertBinary<T> for VecDeque<T> {}
