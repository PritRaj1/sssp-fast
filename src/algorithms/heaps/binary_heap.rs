use crate::utils::FloatNumber;
use std::cmp::Ordering;
use std::collections::BinaryHeap as StdBinaryHeap;

use super::traits::{HeapEntry, PriorityQueue};

#[derive(Clone, Copy, Debug)]
struct MinHeapEntry<T: FloatNumber>(HeapEntry<T>);

impl<T: FloatNumber> PartialEq for MinHeapEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.dist == other.0.dist && self.0.vertex == other.0.vertex
    }
}

impl<T: FloatNumber> Eq for MinHeapEntry<T> {}

impl<T: FloatNumber> PartialOrd for MinHeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: FloatNumber> Ord for MinHeapEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.0.dist.partial_cmp(&self.0.dist) {
            Some(ord) => ord,
            None => Ordering::Equal,
        }
    }
}

#[derive(Debug)]
pub struct BinaryHeap<T: FloatNumber> {
    heap: StdBinaryHeap<MinHeapEntry<T>>,
}

impl<T: FloatNumber> BinaryHeap<T> {
    pub fn new() -> Self {
        Self {
            heap: StdBinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: StdBinaryHeap::with_capacity(capacity),
        }
    }
}

impl<T: FloatNumber> Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatNumber> PriorityQueue<T> for BinaryHeap<T> {
    fn new() -> Self {
        BinaryHeap::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        BinaryHeap::with_capacity(capacity)
    }

    #[inline]
    fn push(&mut self, dist: T, vertex: usize) {
        self.heap.push(MinHeapEntry(HeapEntry::new(dist, vertex)));
    }

    #[inline]
    fn pop(&mut self) -> Option<HeapEntry<T>> {
        self.heap.pop().map(|e| e.0)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
        self.heap.len()
    }

    fn clear(&mut self) {
        self.heap.clear();
    }
}
