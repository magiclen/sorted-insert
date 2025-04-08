use core::cmp::Ordering;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{
    SortedInsertArcRwLock, SortedInsertArcRwLockBasic, SortedInsertArcRwLockBy,
    SortedInsertArcRwLockByKey, SortedInsertBinaryArcRwLock, SortedInsertBinaryArcRwLockBy,
    SortedInsertBinaryArcRwLockByKey,
};

impl<T> SortedInsertArcRwLockBasic<T> for VecDeque<Arc<RwLock<T>>> {
    #[inline]
    fn insert_element(&mut self, index: usize, element: Arc<RwLock<T>>) {
        self.insert(index, element);
    }
}

impl<T> SortedInsertArcRwLockBy<T> for VecDeque<Arc<RwLock<T>>> {
    #[inline]
    fn get_sorted_insert_index_by<F: FnMut(&Arc<RwLock<T>>) -> bool>(&self, f: F) -> usize {
        match self.iter().rposition(f) {
            Some(i) => i + 1,
            None => 0,
        }
    }
}

impl<T> SortedInsertArcRwLockByKey<T> for VecDeque<Arc<RwLock<T>>> {}

impl<T: Ord> SortedInsertArcRwLock<T> for VecDeque<Arc<RwLock<T>>> {}

impl<T> SortedInsertBinaryArcRwLockBy<T> for VecDeque<Arc<RwLock<T>>> {
    #[inline]
    fn get_sorted_insert_index_binary_by<F: FnMut(&Arc<RwLock<T>>) -> Ordering>(
        &mut self,
        f: F,
    ) -> usize {
        match self.make_contiguous().binary_search_by(f) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }
}

impl<T> SortedInsertBinaryArcRwLockByKey<T> for VecDeque<Arc<RwLock<T>>> {}

impl<T: Ord> SortedInsertBinaryArcRwLock<T> for VecDeque<Arc<RwLock<T>>> {}
