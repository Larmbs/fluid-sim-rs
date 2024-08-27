//! Module which defines a variety of display methods

use super::flow_box::FlowBox;
use macroquad::prelude::*;

/// Display modes of flow
pub enum DisplayMode {
    VelocityBlackWhite,
}

/// Display flags
pub mod flags {
    pub const SHOW_VELOCITY_VECTORS: u8 = 0b0001;
}

/// Displays a flow box given a mode
pub struct FlowDisplay {
    mode: DisplayMode,
    flags: u8,
}
impl FlowDisplay {
    pub fn init(mode: DisplayMode, flags: u8) -> Self {
        Self { mode, flags }
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

                if self.flags & flags::SHOW_VELOCITY_VECTORS == 1 {
                    let x1 = x as f32 * block_size + block_size / 2.;
                    let y1 = y as f32 * block_size + block_size / 2.;

                    let x2 = x1 + flow_box.vel_x[FlowBox::index(x, y, &dim)] as f32;
                    let y2 = y1 + flow_box.vel_y[FlowBox::index(x, y, &dim)] as f32;

                    draw_line(x1, y1, x2, y2, 1.0, WHITE)
                }
            }
        }
    }
}
