use core::cmp::Ordering;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    SortedInsertArcMutex, SortedInsertArcMutexBasic, SortedInsertArcMutexBy,
    SortedInsertArcMutexByKey, SortedInsertBinaryArcMutex, SortedInsertBinaryArcMutexBy,
    SortedInsertBinaryArcMutexByKey,
};

impl<T> SortedInsertArcMutexBasic<T> for VecDeque<Arc<Mutex<T>>> {
    #[inline]
    fn insert_element(&mut self, index: usize, element: Arc<Mutex<T>>) {
        self.insert(index, element);
    }
}

impl<T> SortedInsertArcMutexBy<T> for VecDeque<Arc<Mutex<T>>> {
    #[inline]
    fn get_sorted_insert_index_by<F: FnMut(&Arc<Mutex<T>>) -> bool>(&self, f: F) -> usize {
        match self.iter().rposition(f) {
            Some(i) => i + 1,
            None => 0,
        }
    }
}

impl<T> SortedInsertArcMutexByKey<T> for VecDeque<Arc<Mutex<T>>> {}

impl<T: Ord> SortedInsertArcMutex<T> for VecDeque<Arc<Mutex<T>>> {}

impl<T> SortedInsertBinaryArcMutexBy<T> for VecDeque<Arc<Mutex<T>>> {
    #[inline]
    fn get_sorted_insert_index_binary_by<F: FnMut(&Arc<Mutex<T>>) -> Ordering>(
        &mut self,
        f: F,
    ) -> usize {
        match self.make_contiguous().binary_search_by(f) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }
}

impl<T> SortedInsertBinaryArcMutexByKey<T> for VecDeque<Arc<Mutex<T>>> {}

impl<T: Ord> SortedInsertBinaryArcMutex<T> for VecDeque<Arc<Mutex<T>>> {}
