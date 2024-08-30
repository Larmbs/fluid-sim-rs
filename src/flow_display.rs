//! Module which defines a variety of display methods

use super::flow_box::FlowBox;
use lazy_static::lazy_static;
use macroquad::prelude::*;
/// Display modes of flow
pub enum DisplayMode {
    VelocityBlackWhite,
}

/// Display flags
pub mod flags {
    pub const NONE: u8 = 0b0000;
    pub const SHOW_VELOCITY_VECTORS: u8 = 0b0001;
}

lazy_static! {
    static ref SPEED_COLORS: [Color; 3] = [
        Color::from_hex(0x80ff2b),
        Color::from_hex(0xffd429),
        Color::from_hex(0xff3729)
    ];
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

                    let vx =
                        (flow_box.vel_x[FlowBox::index(x, y, &dim)] * 80.0).clamp(-6.0, 6.0) as f32;
                    let vy =
                        (flow_box.vel_y[FlowBox::index(x, y, &dim)] * 80.0).clamp(-6.0, 6.0) as f32;
                    let mag_sq = vx.powi(2) + vy.powi(2);
                    let scalar = mag_sq / 36.0;
                    let color = Color::from_vec(match scalar {
                        0.0..=1.0 => SPEED_COLORS[0]
                            .to_vec()
                            .lerp(SPEED_COLORS[1].to_vec(), scalar),
                        1.0..=2.0 => SPEED_COLORS[1]
                            .to_vec()
                            .lerp(SPEED_COLORS[2].to_vec(), scalar - 1.0),
                        _ => SPEED_COLORS[2].to_vec(),
                    });
                    draw_line(x1, y1, x1 + vx, y1 + vy, 1.0, color)
                }
            }
        }
    }
}
