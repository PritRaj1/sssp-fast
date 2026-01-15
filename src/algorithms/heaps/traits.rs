use crate::utils::FloatNumber;
use std::fmt::Debug;

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

/// Min-heap interface. `pop()` returns smallest distance.
pub trait PriorityQueue<T: FloatNumber>: Default + Debug {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, dist: T, vertex: usize);
    fn pop(&mut self) -> Option<HeapEntry<T>>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn clear(&mut self);
}
