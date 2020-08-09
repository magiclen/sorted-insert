use std::collections::VecDeque;

use crate::SortedInsert;

#[cfg(feature = "nightly")]
use crate::SortedInsertBinary;

impl<T: Ord> SortedInsert<T> for VecDeque<T> {
    #[inline]
    fn sorted_insert_asc(&mut self, element: T) -> usize {
        match self.iter().rposition(|e| e <= &element) {
            Some(mut i) => {
                i += 1;

                self.insert(i, element);

                i
            }
            None => {
                self.insert(0, element);

                0
            }
        }
    }

    #[inline]
    fn sorted_insert_desc(&mut self, element: T) -> usize {
        match self.iter().rposition(|e| e >= &element) {
            Some(mut i) => {
                i += 1;

                self.insert(i, element);

                i
            }
            None => {
                self.insert(0, element);

                0
            }
        }
    }
}

#[cfg(feature = "nightly")]
impl<T: Ord> SortedInsertBinary<T> for VecDeque<T> {
    #[inline]
    fn sorted_insert_asc_binary(&mut self, element: T) -> usize {
        let i = match self.make_contiguous().binary_search(&element) {
            Ok(i) | Err(i) => i,
        };

        self.insert(i, element);

        i
    }

    #[inline]
    fn sorted_insert_desc_binary(&mut self, element: T) -> usize {
        let i = match self.make_contiguous().binary_search_by(|e| element.cmp(e)) {
            Ok(i) | Err(i) => i,
        };

        self.insert(i, element);

        i
    }
}
