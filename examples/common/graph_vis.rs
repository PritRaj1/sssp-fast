use super::graphs::EuclideanGraph;

#[derive(Clone, Copy, PartialEq)]
pub enum NodeState {
    Unvisited,
    InQueue,
    Visited,
    Path,
    Start,
    End,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EdgeState {
    Default,
    Visited,
    Path,
}

/// Graph state at each step
#[derive(Clone)]
pub struct GraphVisState {
    pub node_states: Vec<NodeState>,
    pub edge_states: Vec<Vec<EdgeState>>,
    pub visit_order: Vec<Option<usize>>,
    pub max_visited: usize,
    pub graph: EuclideanGraph,
    pub start: usize,
    pub end: usize,
}

impl GraphVisState {
    pub fn new(graph: &EuclideanGraph, start: usize, end: usize) -> Self {
        let n = graph.n_vertices();
        let mut node_states = vec![NodeState::Unvisited; n];
        node_states[start] = NodeState::Start;
        node_states[end] = NodeState::End;

        // Init edge states
        let mut edge_states = vec![vec![EdgeState::Default; n]; n];
        for (u, neighbors) in graph.adjacency.iter().enumerate() {
            for &(v, _) in neighbors {
                edge_states[u][v] = EdgeState::Default;
            }
        }

        Self {
            node_states,
            edge_states,
            visit_order: vec![None; n],
            max_visited: 0,
            graph: graph.clone(),
            start,
            end,
        }
    }

    pub fn mark_in_queue(&mut self, vertex: usize) {
        if self.node_states[vertex] == NodeState::Unvisited {
            self.node_states[vertex] = NodeState::InQueue;
        }
    }

    pub fn mark_visited(&mut self, vertex: usize, parent: Option<usize>) {
        if self.node_states[vertex] != NodeState::Start
            && self.node_states[vertex] != NodeState::End
        {
            self.node_states[vertex] = NodeState::Visited;
        }
        self.visit_order[vertex] = Some(self.max_visited);
        self.max_visited += 1;

        // Mark edge from parent as visited
        if let Some(p) = parent {
            self.edge_states[p][vertex] = EdgeState::Visited;
            self.edge_states[vertex][p] = EdgeState::Visited;
        }
    }

    pub fn mark_path(&mut self, path: &[usize]) {
        for window in path.windows(2) {
            let u = window[0];
            let v = window[1];
            self.edge_states[u][v] = EdgeState::Path;
            self.edge_states[v][u] = EdgeState::Path;
        }

        for &vertex in path {
            if self.node_states[vertex] != NodeState::Start
                && self.node_states[vertex] != NodeState::End
            {
                self.node_states[vertex] = NodeState::Path;
            }
        }
    }
}
