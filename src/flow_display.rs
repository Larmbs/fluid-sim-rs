//! Module which defines a variety of display methods

use super::flow_box::FlowBox;
use macroquad::prelude::*;

/// Display modes of flow
pub enum DisplayMode {
    VelocityBlackWhite,
}

/// Displays a flow box given a mode
pub struct FlowDisplay {
    mode: DisplayMode,
}
impl FlowDisplay {
    pub fn init(mode: DisplayMode) -> Self {
        Self { mode }
    }
    pub fn display(&self, flow_box: &FlowBox) {
        let dim = flow_box.dim;

        let block_size = (screen_width() / dim.0 as f32).min(screen_height() / dim.1 as f32);

        for x in 0..dim.0 {
            for y in 0..dim.1 {
                match self.mode {
                    DisplayMode::VelocityBlackWhite => {
                        let mag =
                            flow_box.density[FlowBox::index(x, y, &dim)].clamp(0.0, 1.0) as f32;
                        let color = Color {
                            r: mag,
                            g: mag,
                            b: mag,
                            a: 1.0,
                        };
                        draw_rectangle(
                            x as f32 * block_size,
                            y as f32 * block_size,
                            block_size,
                            block_size,
                            color,
                        )
                    }
                }
            }
        }
    }
}
