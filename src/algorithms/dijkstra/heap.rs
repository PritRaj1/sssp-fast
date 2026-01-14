use crate::utils::FloatNumber;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Min-heap entry: (distance, vertex).
#[derive(Clone, Copy, Debug)]
pub struct HeapEntry<T: FloatNumber> {
    pub dist: T,
    pub vertex: usize,
}

impl<T: FloatNumber> HeapEntry<T> {
    #[inline]
    pub fn new(dist: T, vertex: usize) -> Self {
        Self { dist, vertex }
    }
}

/// Reverse ordering for min-heap (BinaryHeap is max-heap by default).
impl<T: FloatNumber> PartialEq for HeapEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.vertex == other.vertex
    }
}

impl<T: FloatNumber> Eq for HeapEntry<T> {}

impl<T: FloatNumber> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Reverse order: smaller distance = higher priority
impl<T: FloatNumber> Ord for HeapEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.dist.partial_cmp(&self.dist) {
            Some(ord) => ord,
            None => Ordering::Equal, // Handle NaN
        }
    }
}

/// Min-heap for priority queue operations.
#[derive(Debug)]
pub struct MinHeap<T: FloatNumber> {
    heap: BinaryHeap<HeapEntry<T>>,
}

impl<T: FloatNumber> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(&mut self, dist: T, vertex: usize) {
        self.heap.push(HeapEntry::new(dist, vertex));
    }

    #[inline]
    pub fn pop(&mut self) -> Option<HeapEntry<T>> {
        self.heap.pop()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: FloatNumber> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
