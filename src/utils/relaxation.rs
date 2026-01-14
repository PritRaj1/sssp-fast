use super::graph::FloatNumber;

/// Edge relaxation effect on distance.
#[derive(Clone, Copy, Debug)]
pub enum RelaxResult {
    Improved,
    NoChange,
}

/// Relax an edge (u -> v) with weight w.
#[inline]
pub fn relax<T: FloatNumber>(
    dist: &mut [T],
    parent: &mut [usize],
    u: usize,
    v: usize,
    w: T,
) -> RelaxResult {
    let new_dist = dist[u] + w;
    if new_dist < dist[v] {
        dist[v] = new_dist;
        parent[v] = u;
        RelaxResult::Improved
    } else {
        RelaxResult::NoChange
    }
}

/// Relax an edge with known distance from source to u.
#[inline]
pub fn relax_with<T: FloatNumber>(
    dist: &mut [T],
    parent: &mut [usize],
    u: usize,
    d_u: T,
    v: usize,
    w: T,
) -> RelaxResult {
    let new_dist = d_u + w;
    if new_dist < dist[v] {
        dist[v] = new_dist;
        parent[v] = u;
        RelaxResult::Improved
    } else {
        RelaxResult::NoChange
    }
}
