mod collections;

use core::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[doc(hidden)]
pub trait SortedInsertArcMutexBasic<T> {
    #[doc(hidden)]
    fn insert_element(&mut self, index: usize, element: Arc<Mutex<T>>);
}

pub trait SortedInsertArcMutexBy<T>: SortedInsertArcMutexBasic<T> {
    /// Insert elements to this sorted collection by a specific comparator and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_by<F: FnMut(&Arc<Mutex<T>>, &T) -> bool>(
        &mut self,
        element: Arc<Mutex<T>>,
        mut f: F,
    ) -> usize {
        let element_guard = element.lock().unwrap();

        let index = self.get_sorted_insert_index_by(|e| f(e, &*element_guard));

        drop(element_guard);

        self.insert_element(index, element);

        index
    }

    #[doc(hidden)]
    fn get_sorted_insert_index_by<F: FnMut(&Arc<Mutex<T>>) -> bool>(&self, f: F) -> usize;
}

pub trait SortedInsertArcMutexByKey<T>: SortedInsertArcMutexBy<T> {
    /// Insert elements to this sorted collection in ascending order by a specific key and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_asc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<Mutex<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_by(element, |e, element| {
            let e_guard = e.lock().unwrap();

            f(&*e_guard) <= f(element)
        })
    }

    /// Insert elements to this sorted collection in descending order by a specific key and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_desc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<Mutex<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_by(element, |e, element| {
            let e_guard = e.lock().unwrap();

            f(&*e_guard) >= f(element)
        })
    }
}

pub trait SortedInsertArcMutex<T: Ord>: SortedInsertArcMutexByKey<T> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_asc(&mut self, element: Arc<Mutex<T>>) -> usize {
        self.sorted_insert_asc_by_key(element, |element| element)
    }

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use linear search to find the index where a matching element could be inserted.
    fn sorted_insert_desc(&mut self, element: Arc<Mutex<T>>) -> usize {
        self.sorted_insert_desc_by_key(element, |element| element)
    }
}

pub trait SortedInsertBinaryArcMutexBy<T>: SortedInsertArcMutexBy<T> {
    /// Insert elements to this sorted collection by a specific comparator and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    fn sorted_insert_binary_by<F: FnMut(&Arc<Mutex<T>>, &T) -> Ordering>(
        &mut self,
        element: Arc<Mutex<T>>,
        mut f: F,
    ) -> usize {
        let element_guard = element.lock().unwrap();

        let index = self.get_sorted_insert_index_binary_by(|e| f(e, &*element_guard));

        drop(element_guard);

        self.insert_element(index, element);

        index
    }

    #[doc(hidden)]
    fn get_sorted_insert_index_binary_by<F: FnMut(&Arc<Mutex<T>>) -> Ordering>(
        &mut self,
        f: F,
    ) -> usize;
}

pub trait SortedInsertBinaryArcMutexByKey<T>: SortedInsertBinaryArcMutexBy<T> {
    /// Insert elements to this sorted collection in ascending order by a specific key and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_binary_asc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<Mutex<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_binary_by(element, |e, element| {
            let e_guard = e.lock().unwrap();

            f(&*e_guard).cmp(f(element))
        })
    }

    /// Insert elements to this sorted collection in descending order by a specific key and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_binary_desc_by_key<A: Ord, F: FnMut(&T) -> &A>(
        &mut self,
        element: Arc<Mutex<T>>,
        mut f: F,
    ) -> usize {
        self.sorted_insert_binary_by(element, |e, element| {
            let e_guard = e.lock().unwrap();

            f(element).cmp(f(&*e_guard))
        })
    }
}

pub trait SortedInsertBinaryArcMutex<T: Ord>: SortedInsertBinaryArcMutexByKey<T> {
    /// Insert elements to this sorted collection in ascending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_asc_binary(&mut self, element: Arc<Mutex<T>>) -> usize {
        self.sorted_insert_binary_asc_by_key(element, |element| element)
    }

    /// Insert elements to this sorted collection in descending order and return the inserted index. Use binary search to find the index where a matching element could be inserted.
    #[inline]
    fn sorted_insert_desc_binary(&mut self, element: Arc<Mutex<T>>) -> usize {
        self.sorted_insert_binary_desc_by_key(element, |element| element)
    }
}
