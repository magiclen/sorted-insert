mod collections;

use core::cmp::Ordering;
use std::sync::{Arc, RwLock};

#[doc(hidden)]
pub trait SortedInsertArcRwLockBasic<T> {
    #[doc(hidden)]
    fn insert_element(&mut self, index: usize, element: Arc<RwLock<T>>);
}

pub trait SortedInsertArcRwLockBy<T>: SortedInsertArcRwLockBasic<T> {
    /// Insert elements to this sorted collection by a specific comparator and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_by<F: FnMut(&Arc<RwLock<T>>, &T) -> bool>(
        &mut self,
        element: Arc<RwLock<T>>,
        mut f: F,
    ) -> usize {
        let element_guard = element.read().unwrap();

        let index = self.get_sorted_insert_index_by(|e| f(e, &*element_guard));

        drop(element_guard);

        self.insert_element(index, element);

        index
    }

    #[doc(hidden)]
    fn get_sorted_insert_index_by<F: FnMut(&Arc<RwLock<T>>) -> bool>(&self, f: F) -> usize;
}

pub trait SortedInsertArcRwLockByKey<T>: SortedInsertArcRwLockBy<T> {
    /// Insert elements to this sorted collection in ascending order by a specific key and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_asc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<RwLock<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_by(element.clone(), |e, element_t| {
            let e_guard = e.read().unwrap();

            f(&*e_guard) <= f(element_t)
        })
    }

    /// Insert elements to this sorted collection in descending order by a specific key and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_desc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<RwLock<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_by(element.clone(), |e, element_t| {
            let e_guard = e.read().unwrap();

            f(&*e_guard) >= f(element_t)
        })
    }
}

pub trait SortedInsertArcRwLock<T: Ord>: SortedInsertArcRwLockByKey<T> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_asc(&mut self, element: Arc<RwLock<T>>) -> usize {
        self.sorted_insert_asc_by_key(element, |element| element)
    }

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_desc(&mut self, element: Arc<RwLock<T>>) -> usize {
        self.sorted_insert_desc_by_key(element, |element| element)
    }
}

pub trait SortedInsertBinaryArcRwLockBy<T>: SortedInsertArcRwLockBy<T> {
    /// Insert elements to this sorted collection by a specific comparator and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    fn sorted_insert_binary_by<F: FnMut(&Arc<RwLock<T>>, &T) -> Ordering>(
        &mut self,
        element: Arc<RwLock<T>>,
        mut f: F,
    ) -> usize {
        let element_guard = element.read().unwrap();

        let index = self.get_sorted_insert_index_binary_by(|e| f(e, &*element_guard));

        drop(element_guard);

        self.insert_element(index, element);

        index
    }

    #[doc(hidden)]
    fn get_sorted_insert_index_binary_by<F: FnMut(&Arc<RwLock<T>>) -> Ordering>(
        &mut self,
        f: F,
    ) -> usize;
}

pub trait SortedInsertBinaryArcRwLockByKey<T>: SortedInsertBinaryArcRwLockBy<T> {
    /// Insert elements to this sorted collection in ascending order by a specific key and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_binary_asc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<RwLock<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_binary_by(element.clone(), |e, element_t| {
            let e_guard = e.read().unwrap();

            f(&*e_guard).cmp(f(element_t))
        })
    }

    /// Insert elements to this sorted collection in descending order by a specific key and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_binary_desc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<RwLock<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_binary_by(element.clone(), |e, element_t| {
            let e_guard = e.read().unwrap();

            f(element_t).cmp(f(&*e_guard))
        })
    }
}

pub trait SortedInsertBinaryArcRwLock<T: Ord>: SortedInsertBinaryArcRwLockByKey<T> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_asc_binary(&mut self, element: Arc<RwLock<T>>) -> usize {
        self.sorted_insert_binary_asc_by_key(element, |element| element)
    }

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    ///
    /// ## Safety
    ///
    /// This function will panic if the element is locked.
    #[inline]
    fn sorted_insert_desc_binary(&mut self, element: Arc<RwLock<T>>) -> usize {
        self.sorted_insert_binary_desc_by_key(element, |element| element)
    }
}
