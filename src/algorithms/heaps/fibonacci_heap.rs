use crate::utils::FloatNumber;
use std::mem;

use super::traits::{HeapEntry, PriorityQueue};

#[derive(Debug)]
struct FibNode<T: FloatNumber> {
    entry: HeapEntry<T>,
    degree: usize,
    child: Option<Box<FibNode<T>>>,
    sibling: Option<Box<FibNode<T>>>,
}

impl<T: FloatNumber> FibNode<T> {
    fn new(dist: T, vertex: usize) -> Self {
        Self {
            entry: HeapEntry::new(dist, vertex),
            degree: 0,
            child: None,
            sibling: None,
        }
    }

    fn link_child(&mut self, mut other: Box<FibNode<T>>) {
        other.sibling = self.child.take();
        self.child = Some(other);
        self.degree += 1;
    }
}

#[derive(Debug)]
pub struct FibonacciHeap<T: FloatNumber> {
    roots: Option<Box<FibNode<T>>>,
    min_node: Option<*mut FibNode<T>>,
    len: usize,
}

impl<T: FloatNumber> FibonacciHeap<T> {
    pub fn new() -> Self {
        Self {
            roots: None,
            min_node: None,
            len: 0,
        }
    }

    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    fn add_to_roots(&mut self, mut node: Box<FibNode<T>>) {
        let node_ptr = node.as_mut() as *mut FibNode<T>;

        match self.min_node {
            None => {
                self.min_node = Some(node_ptr);
            }
            Some(min_ptr) => {
                let min_dist = unsafe { (*min_ptr).entry.dist };
                if node.entry.dist < min_dist {
                    self.min_node = Some(node_ptr);
                }
            }
        }

        node.sibling = self.roots.take();
        self.roots = Some(node);

        if let Some(min_ptr) = self.min_node {
            if min_ptr == node_ptr {
                self.min_node = Some(self.roots.as_mut().unwrap().as_mut() as *mut FibNode<T>);
            }
        }
    }

    fn consolidate(&mut self) {
        if self.roots.is_none() {
            return;
        }

        let max_degree = (self.len as f64).log2().ceil() as usize + 2;
        let mut degree_table: Vec<Option<Box<FibNode<T>>>> =
            (0..=max_degree).map(|_| None).collect();

        let mut current = self.roots.take();
        while let Some(mut node) = current {
            current = node.sibling.take();
            let mut d = node.degree;

            while d < degree_table.len() && degree_table[d].is_some() {
                let mut other = degree_table[d].take().unwrap();
                if other.entry.dist < node.entry.dist {
                    mem::swap(&mut node, &mut other);
                }
                node.link_child(other);
                d += 1;
            }

            if d >= degree_table.len() {
                degree_table.resize_with(d + 1, || None);
            }
            degree_table[d] = Some(node);
        }

        self.roots = None;
        self.min_node = None;

        for node in degree_table.into_iter().flatten() {
            self.add_to_roots(node);
        }
    }
}

impl<T: FloatNumber> Default for FibonacciHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatNumber> PriorityQueue<T> for FibonacciHeap<T> {
    fn new() -> Self {
        FibonacciHeap::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        FibonacciHeap::with_capacity(capacity)
    }

    #[inline]
    fn push(&mut self, dist: T, vertex: usize) {
        let node = Box::new(FibNode::new(dist, vertex));
        self.add_to_roots(node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<HeapEntry<T>> {
        if self.len == 0 {
            return None;
        }

        let mut prev: Option<*mut FibNode<T>> = None;
        let mut current_ptr = self.roots.as_mut().map(|n| n.as_mut() as *mut FibNode<T>);
        let mut min_prev: Option<*mut FibNode<T>> = None;
        let min_ptr = self.min_node?;

        while let Some(curr) = current_ptr {
            if curr == min_ptr {
                min_prev = prev;
                break;
            }
            prev = Some(curr);
            current_ptr = unsafe {
                (*curr)
                    .sibling
                    .as_mut()
                    .map(|n| n.as_mut() as *mut FibNode<T>)
            };
        }

        let min_node = if let Some(prev_ptr) = min_prev {
            unsafe {
                let prev_node = &mut *prev_ptr;
                let mut min = prev_node.sibling.take().unwrap();
                prev_node.sibling = min.sibling.take();
                min
            }
        } else {
            let mut min = self.roots.take().unwrap();
            self.roots = min.sibling.take();
            min
        };

        let result = min_node.entry;

        let mut child = min_node.child;
        while let Some(mut c) = child {
            child = c.sibling.take();
            self.add_to_roots(c);
        }

        self.len -= 1;

        if self.len > 0 {
            self.consolidate();
        } else {
            self.min_node = None;
        }

        Some(result)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    fn clear(&mut self) {
        self.roots = None;
        self.min_node = None;
        self.len = 0;
    }
}
