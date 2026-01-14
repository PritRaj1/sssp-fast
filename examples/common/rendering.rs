use plotters::prelude::*;

pub const CELL_SIZE: u32 = 25;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Wall,
    Unvisited,
    InQueue,
    Visited,
    Path,
    Start,
    End,
}

pub fn get_color_palette() -> Vec<[u8; 3]> {
    let mut palette = Vec::with_capacity(256);

    palette.push([0, 0, 0]); // 0: Wall - black
    palette.push([60, 60, 60]); // 1: Unvisited - grey
    palette.push([255, 200, 50]); // 2: In queue/frontier - orange
    palette.push([100, 180, 255]); // 3: Visited - light blue
    palette.push([50, 255, 100]); // 4: Path - green
    palette.push([255, 80, 80]); // 5: Start - red
    palette.push([180, 50, 255]); // 6: End - purple
    palette.push([255, 255, 255]); // 7: Grid lines - white
    palette.push([40, 40, 40]); // 8: Background - dark grey

    // Gradient for visited cells from dark blue (early) to cyan (late)
    for i in 0..64 {
        let t = i as f32 / 63.0;
        let r = (30.0 + t * 70.0) as u8;
        let g = (80.0 + t * 120.0) as u8;
        let b = (180.0 + t * 75.0) as u8;
        palette.push([r, g, b]);
    }

    while palette.len() < 256 {
        palette.push([0, 0, 0]);
    }

    palette
}

pub fn cell_state_to_palette_index(state: CellState) -> u8 {
    match state {
        CellState::Wall => 0,
        CellState::Unvisited => 1,
        CellState::InQueue => 2,
        CellState::Visited => 3,
        CellState::Path => 4,
        CellState::Start => 5,
        CellState::End => 6,
    }
}

pub fn visited_gradient_index(order: usize, max_order: usize) -> u8 {
    let t = if max_order > 0 {
        (order as f32 / max_order as f32).min(1.0)
    } else {
        0.0
    };
    9 + (t * 63.0) as u8
}

pub struct RenderParams<'a> {
    pub grid: &'a [Vec<CellState>],
    pub visit_order: &'a [Vec<Option<usize>>],
    pub max_visited: usize,
    pub title: &'a str,
}

pub fn render_frame(
    png_path: &str,
    params: RenderParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let rows = params.grid.len();
    let cols = params.grid[0].len();
    let width = cols as u32 * CELL_SIZE;
    let height = rows as u32 * CELL_SIZE + 40;
    let palette = get_color_palette();

    {
        let root = BitMapBackend::new(png_path, (width, height)).into_drawing_area();
        root.fill(&RGBColor(40, 40, 50))?;

        root.draw(&Text::new(
            params.title,
            (10, 10),
            ("sans-serif", 20).into_font().color(&WHITE),
        ))?;

        let plot_area = root.margin(40, 0, 0, 0);

        for row in 0..rows {
            for col in 0..cols {
                let state = params.grid[row][col];
                let x0 = col as i32 * CELL_SIZE as i32;
                let y0 = row as i32 * CELL_SIZE as i32;
                let x1 = x0 + CELL_SIZE as i32;
                let y1 = y0 + CELL_SIZE as i32;

                let color_idx = if state == CellState::Visited {
                    if let Some(order) = params.visit_order[row][col] {
                        visited_gradient_index(order, params.max_visited)
                    } else {
                        cell_state_to_palette_index(state)
                    }
                } else {
                    cell_state_to_palette_index(state)
                };

                let [r, g, b] = palette[color_idx as usize];
                plot_area.draw(&Rectangle::new(
                    [(x0, y0), (x1, y1)],
                    RGBColor(r, g, b).filled(),
                ))?;

                plot_area.draw(&Rectangle::new(
                    [(x0, y0), (x1, y1)],
                    RGBColor(80, 80, 90).stroke_width(1),
                ))?;
            }
        }

        root.present()?;
    }

    Ok(())
}
