use crate::trait::FloatNum;
use nalgebra::usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge<T: FloatNum> {
    pub to: usize,
    pub w: T,
}

pub trait Graph<T: FloatNum> {
    fn n(&self) -> usize;
    fn for_each_out_edge(&self, u: usize, f: impl FnMut(usize, T));
}

#[derive(Clone, Debug)]
pub struct AdjListGraph<T: FloatNum> {
    n: usize,
    adj: Vec<Vec<Edge<T>>>,
}

impl<T: FloatNum> AdjListGraph<T> {
    pub fn new(n: usize) -> Self {
        Self { n, adj: vec![Vec::new(); n] }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: T) {
        assert!(u < self.n && v < self.n);
        self.adj[u].push(Edge { to: v, w });
    }

    pub fn neighbors(&self, u: usize) -> &[Edge<T>] {
        &self.adj[u]
    }
}

impl<T: FloatNum> Graph<T> for AdjListGraph<T> {
    fn n(&self) -> usize { self.n }

    fn for_each_out_edge(&self, u: usize, mut f: impl FnMut(usize, T)) {
        for e in &self.adj[u] {
            f(e.to, e.w);
        }
    }
}
