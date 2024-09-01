//! Module which defines a variety of display methods

use core::f32;

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
    pub fn get_mouse_cord(&self, flow_box: &FlowBox) -> (usize, usize) {
        let dim = flow_box.dim;
        let block_size = (screen_width() / dim.0 as f32).min(screen_height() / dim.1 as f32);

        let mouse_pos = mouse_position();
        let x = (mouse_pos.0 / block_size) as usize;
        let y = (mouse_pos.1 / block_size) as usize;

        let clamped_x = x.clamp(0, dim.0);
        let clamped_y = y.clamp(0, dim.1);

        (clamped_x, clamped_y)
    }
    pub fn get_mouse_delta_angle(&self) -> f32 {
        let mouse_delta = mouse_delta_position();
        let angle = mouse_delta.angle_between(Vec2::from_angle(0.0));
        if angle.is_finite() {
            angle
        } else {
            0.0
        }
    }
    pub fn display(&self, flow_box: &FlowBox) {
        let dim = flow_box.dim;

        let block_size = (screen_width() / dim.0 as f32).min(screen_height() / dim.1 as f32);
        let half_block_size = block_size / 2.0;

        for x in 0..dim.0 {
            let screen_x = x as f32 * block_size;
            for y in 0..dim.1 {
                let screen_y = y as f32 * block_size;

                match self.mode {
                    DisplayMode::VelocityBlackWhite => {
                        let color = Color {
                            r: flow_box.red_density[FlowBox::index(&x, &y, &dim)].clamp(0.0, 1.0) as f32,
                            g: flow_box.green_density[FlowBox::index(&x, &y, &dim)].clamp(0.0, 1.0) as f32,
                            b: flow_box.blue_density[FlowBox::index(&x, &y, &dim)].clamp(0.0, 1.0) as f32,
                            a: 1.0,
                        };

                        draw_rectangle(screen_x, screen_y, block_size, block_size, color)
                    }
                }

                if self.flags & flags::SHOW_VELOCITY_VECTORS == 1 {
                    let x1 = screen_x + half_block_size;
                    let y1 = screen_y + half_block_size;

                    let vx = (flow_box.vel_x[FlowBox::index(&x, &y, &dim)] * 80.0).clamp(-6.0, 6.0)
                        as f32;
                    let vy = (flow_box.vel_y[FlowBox::index(&x, &y, &dim)] * 80.0).clamp(-6.0, 6.0)
                        as f32;
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
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, WHITE);
    }
}
