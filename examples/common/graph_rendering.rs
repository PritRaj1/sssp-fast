use plotters::prelude::*;

use super::graph_vis::{EdgeState, GraphVisState, NodeState};

pub const GRAPH_WIDTH: u32 = 800;
pub const GRAPH_HEIGHT: u32 = 800;
pub const TITLE_HEIGHT: u32 = 40;
pub const NODE_RADIUS: i32 = 4;

fn node_color(state: NodeState) -> RGBColor {
    match state {
        NodeState::Unvisited => RGBColor(80, 80, 80),  // grey
        NodeState::InQueue => RGBColor(255, 200, 50),  // orange
        NodeState::Visited => RGBColor(100, 180, 255), // light blue
        NodeState::Path => RGBColor(50, 255, 100),     // green
        NodeState::Start => RGBColor(255, 80, 80),     // red
        NodeState::End => RGBColor(180, 50, 255),      // purple
    }
}

fn visited_gradient_color(order: usize, max_order: usize) -> RGBColor {
    let t = if max_order > 0 {
        (order as f32 / max_order as f32).min(1.0)
    } else {
        0.0
    };
    let r = (30.0 + t * 70.0) as u8;
    let g = (80.0 + t * 120.0) as u8;
    let b = (180.0 + t * 75.0) as u8;
    RGBColor(r, g, b)
}

fn edge_color(state: EdgeState) -> RGBColor {
    match state {
        EdgeState::Default => RGBColor(50, 50, 50),
        EdgeState::Visited => RGBColor(80, 120, 160),
        EdgeState::Path => RGBColor(50, 255, 100),
    }
}

fn edge_width(state: EdgeState) -> u32 {
    match state {
        EdgeState::Default => 1,
        EdgeState::Visited => 1,
        EdgeState::Path => 3,
    }
}

pub struct GraphRenderParams<'a> {
    pub state: &'a GraphVisState,
    pub title: &'a str,
}

pub fn render_graph_frame(
    png_path: &str,
    params: GraphRenderParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = GRAPH_WIDTH;
    let height = GRAPH_HEIGHT + TITLE_HEIGHT;
    let state = params.state;
    let graph = &state.graph;

    let (min_x, min_y, max_x, max_y) = graph.bounds();
    let padding = 0.05;
    let range_x = max_x - min_x + 2.0 * padding;
    let range_y = max_y - min_y + 2.0 * padding;

    let to_screen = |x: f64, y: f64| -> (i32, i32) {
        let sx = ((x - min_x + padding) / range_x * (width - 20) as f64) as i32 + 10;
        let sy = ((y - min_y + padding) / range_y * (height - TITLE_HEIGHT - 20) as f64) as i32
            + TITLE_HEIGHT as i32
            + 10;
        (sx, sy)
    };

    {
        let root = BitMapBackend::new(png_path, (width, height)).into_drawing_area();
        root.fill(&RGBColor(10, 10, 10))?;

        root.draw(&Text::new(
            params.title,
            (10, 10),
            ("sans-serif", 20).into_font().color(&WHITE),
        ))?;

        // Draw edges
        for (u, neighbors) in graph.adjacency.iter().enumerate() {
            let (x1, y1) = graph.positions[u];
            let (sx1, sy1) = to_screen(x1, y1);

            // Draw each edge once
            for &(v, _) in neighbors {
                if u < v {
                    let (x2, y2) = graph.positions[v];
                    let (sx2, sy2) = to_screen(x2, y2);

                    let edge_state = state.edge_states[u][v];
                    let color = edge_color(edge_state);
                    let width = edge_width(edge_state);

                    root.draw(&PathElement::new(
                        vec![(sx1, sy1), (sx2, sy2)],
                        color.stroke_width(width),
                    ))?;
                }
            }
        }

        // Draw nodes on top
        for (i, &(x, y)) in graph.positions.iter().enumerate() {
            let (sx, sy) = to_screen(x, y);
            let node_state = state.node_states[i];

            let color = if node_state == NodeState::Visited {
                if let Some(order) = state.visit_order[i] {
                    visited_gradient_color(order, state.max_visited)
                } else {
                    node_color(node_state)
                }
            } else {
                node_color(node_state)
            };

            root.draw(&Circle::new((sx, sy), NODE_RADIUS, color.filled()))?;

            // Highlight start/end with bigger outline
            if node_state == NodeState::Start || node_state == NodeState::End {
                root.draw(&Circle::new(
                    (sx, sy),
                    NODE_RADIUS + 2,
                    color.stroke_width(2),
                ))?;
            }
        }

        root.present()?;
    }

    Ok(())
}
