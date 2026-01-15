use sssp_fast::{AdjListGraph, FloatNumber};

/// Simple linear chain: 0 -> 1 -> 2 -> ... -> n-1
pub fn linear<T: FloatNumber>(n: usize, weight: T) -> AdjListGraph<T> {
    let mut g = AdjListGraph::new(n);
    for i in 0..n.saturating_sub(1) {
        g.add_edge(i, i + 1, weight);
    }
    g
}

/// Complete graph with uniform weights.
pub fn complete<T: FloatNumber>(n: usize, weight: T) -> AdjListGraph<T> {
    let mut g = AdjListGraph::new(n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                g.add_edge(i, j, weight);
            }
        }
    }
    g
}

/// Diamond graph:
///       1
///      /|\
///     a b c
///    /  |  \
///   0   |   3
///    \  |  /
///     d   f
///      \ /
///       2
/// Returns (graph, optimal_dist_0_to_3)
pub fn diamond<T: FloatNumber>(a: T, b: T, c: T, d: T, f: T) -> (AdjListGraph<T>, T) {
    let mut g = AdjListGraph::new(4);
    g.add_edge(0, 1, a);
    g.add_edge(0, 2, d);
    g.add_edge(1, 2, b);
    g.add_edge(1, 3, c);
    g.add_edge(2, 3, f);

    let path1 = a + c;
    let path2 = a + b + f;
    let path3 = d + f;
    let optimal = if path1 < path2 && path1 < path3 {
        path1
    } else if path2 < path3 {
        path2
    } else {
        path3
    };

    (g, optimal)
}

/// Grid graph (n x m) with uniform weights.
/// Vertices numbered row-major: vertex(r, c) = r * m + c
pub fn grid<T: FloatNumber>(rows: usize, cols: usize, weight: T) -> AdjListGraph<T> {
    let n = rows * cols;
    let mut g = AdjListGraph::new(n);

    for r in 0..rows {
        for c in 0..cols {
            let v = r * cols + c;
            if c + 1 < cols {
                g.add_edge(v, v + 1, weight);
            }
            if r + 1 < rows {
                g.add_edge(v, v + cols, weight);
            }
        }
    }
    g
}

/// Disconnected graph: two components of size n/2 each.
pub fn disconnected<T: FloatNumber>(n: usize, weight: T) -> AdjListGraph<T> {
    let mut g = AdjListGraph::new(n);
    let mid = n / 2;

    for i in 0..mid.saturating_sub(1) {
        g.add_edge(i, i + 1, weight);
    }
    for i in mid..n.saturating_sub(1) {
        g.add_edge(i, i + 1, weight);
    }

    g
}

/// Star graph: center vertex 0 connected to all others.
pub fn star<T: FloatNumber>(n: usize, weight: T) -> AdjListGraph<T> {
    let mut g = AdjListGraph::new(n);
    for i in 1..n {
        g.add_edge(0, i, weight);
    }
    g
}

/// Bidirectional linear chain (undirected behavior).
pub fn linear_undirected<T: FloatNumber>(n: usize, weight: T) -> AdjListGraph<T> {
    let mut g = AdjListGraph::new(n);
    for i in 0..n.saturating_sub(1) {
        add_undirected_edge(&mut g, i, i + 1, weight);
    }
    g
}

/// Add undirected edge (both directions).
pub fn add_undirected_edge<T: FloatNumber>(g: &mut AdjListGraph<T>, u: usize, v: usize, w: T) {
    g.add_edge(u, v, w);
    g.add_edge(v, u, w);
}

/// Undirected grid graph.
pub fn grid_undirected<T: FloatNumber>(rows: usize, cols: usize, weight: T) -> AdjListGraph<T> {
    let n = rows * cols;
    let mut g = AdjListGraph::new(n);
    for r in 0..rows {
        for c in 0..cols {
            let v = r * cols + c;
            if c + 1 < cols {
                add_undirected_edge(&mut g, v, v + 1, weight);
            }
            if r + 1 < rows {
                add_undirected_edge(&mut g, v, v + cols, weight);
            }
        }
    }
    g
}

/// Disconnected undirected graph: two components.
pub fn disconnected_undirected<T: FloatNumber>(n: usize, weight: T) -> AdjListGraph<T> {
    let mut g = AdjListGraph::new(n);
    let mid = n / 2;
    for i in 0..mid.saturating_sub(1) {
        add_undirected_edge(&mut g, i, i + 1, weight);
    }
    for i in mid..n.saturating_sub(1) {
        add_undirected_edge(&mut g, i, i + 1, weight);
    }
    g
}
