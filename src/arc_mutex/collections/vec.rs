use alloc::vec::Vec;
use core::cmp::Ordering;
use std::sync::{Arc, Mutex};

use crate::{
    SortedInsertArcMutex, SortedInsertArcMutexBasic, SortedInsertArcMutexBy,
    SortedInsertArcMutexByKey, SortedInsertBinaryArcMutex, SortedInsertBinaryArcMutexBy,
    SortedInsertBinaryArcMutexByKey,
};

impl<T> SortedInsertArcMutexBasic<T> for Vec<Arc<Mutex<T>>> {
    #[inline]
    fn insert_element(&mut self, index: usize, element: Arc<Mutex<T>>) {
        self.insert(index, element);
    }
}

impl<T> SortedInsertArcMutexBy<T> for Vec<Arc<Mutex<T>>> {
    #[inline]
    fn get_sorted_insert_index_by<F: FnMut(&Arc<Mutex<T>>) -> bool>(&self, f: F) -> usize {
        match self.iter().rposition(f) {
            Some(i) => i + 1,
            None => 0,
        }
    }
}

impl<T> SortedInsertArcMutexByKey<T> for Vec<Arc<Mutex<T>>> {}

impl<T: Ord> SortedInsertArcMutex<T> for Vec<Arc<Mutex<T>>> {}

impl<T> SortedInsertBinaryArcMutexBy<T> for Vec<Arc<Mutex<T>>> {
    #[inline]
    fn get_sorted_insert_index_binary_by<F: FnMut(&Arc<Mutex<T>>) -> Ordering>(
        &mut self,
        f: F,
    ) -> usize {
        match self.binary_search_by(f) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }
}

impl<T> SortedInsertBinaryArcMutexByKey<T> for Vec<Arc<Mutex<T>>> {}

impl<T: Ord> SortedInsertBinaryArcMutex<T> for Vec<Arc<Mutex<T>>> {}
