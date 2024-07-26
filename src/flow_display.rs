//! Module which defines a variety of display methods

use super::flow_box::FlowBox;
use macroquad::prelude::*;

/// Display modes of flow
pub enum DisplayMode {
    VelocityBlackWhite,
}

/// Displays a flow box given a mode
pub struct FlowDisplay;
impl FlowDisplay {
    pub fn display(flow_box: &FlowBox, display_mode: &DisplayMode) {
        let grid_width = flow_box.width;
        let grid_height = flow_box.height;

        let block_size =
            (screen_width() / grid_width as f32).min(screen_height() / grid_height as f32);

        match display_mode {
            DisplayMode::VelocityBlackWhite => {
                for i in 0..flow_box.vec_field.len() {
                    let x = i % grid_width;
                    let y = i / grid_height;

                    let mag = flow_box.vec_field[i].mag() as f32;

                    draw_rectangle(
                        x as f32 * block_size,
                        y as f32 * block_size,
                        block_size,
                        block_size,
                        Color {
                            r: mag.min(1.0),
                            g: mag.min(1.0),
                            b: mag.min(1.0),
                            a: 1.,
                        },
                    )
                }
            }
        }
    }
}
