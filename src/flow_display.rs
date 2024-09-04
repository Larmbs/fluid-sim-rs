//! Defines object capable of drawing a FlowBox

use std::f32::consts::PI;

use super::flow_box::FlowBox;
use lazy_static::lazy_static;
use macroquad::prelude::*;

/// Different viewing modes for FlowBox
#[repr(u8)]
pub enum DisplayMode {
    DensityBlackWhite,
    DensityColor,
    VelocityBlackWhite,
}

/// Flags for debugging fluid sim
pub mod flags {
    pub const NONE: u8 = 0b0000;
    pub const SHOW_VELOCITY_VECTORS: u8 = 0b0001;
    pub const DISPLAY_FPS: u8 = 0b0010;
    pub const FILL_SCREEN: u8 = 0b0100;
}

lazy_static! {
    static ref SPEED_COLORS: [Color; 3] = [
        Color::from_hex(0x80ff2b),
        Color::from_hex(0xffd429),
        Color::from_hex(0xff3729)
    ];
}

/// Displays a FlowBox
pub struct FlowDisplay {
    mode: DisplayMode,
    flags: u8,
    last_d_mouse_angle: f32,
}
impl FlowDisplay {
    pub fn init(mode: DisplayMode, flags: u8) -> FlowDisplay {
        FlowDisplay {
            mode,
            flags,
            last_d_mouse_angle: 0.0,
        }
    }
    /// Changes the display mode
    pub fn set_mode(&mut self, mode: DisplayMode) {
        self.mode = mode;
    }
    /// Sets FluidDisplay flags
    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }
    /// Returns the FlowBox grid coords of mouse
    pub fn get_mouse_cord(&self, dim: &(usize, usize)) -> (usize, usize) {
        let (block_size_x, block_size_y) = self.get_block_size(&dim);
        let mouse_pos: Vec2 = mouse_position().into();
        let pos = mouse_pos / Vec2::new(block_size_x, block_size_y);
        (
            (pos.x as usize).clamp(0, dim.0),
            (pos.y as usize).clamp(0, dim.1),
        )
    }
    /// Returns the last direction mouse was moving in the window
    pub fn get_mouse_mov_dir(&mut self) -> f32 {
        let delta = mouse_delta_position();
        if delta.length_squared() < 0.00001 {
            return self.last_d_mouse_angle;
        };
        let angle = -mouse_delta_position().angle_between(Vec2::from_angle(PI));
        if !angle.is_finite() {
            return self.last_d_mouse_angle;
        };
        self.last_d_mouse_angle = angle;
        angle
    }
    pub fn get_block_size(&self, dim: &(usize, usize)) -> (f32, f32) {
        if self.flags & flags::FILL_SCREEN != 0 {
            (
                screen_width() / dim.0 as f32,
                screen_height() / dim.1 as f32,
            )
        } else {
            let block_size = (screen_width() / dim.0 as f32).min(screen_height() / dim.1 as f32);
            (block_size, block_size)
        }
    }
    /// Displays fluid onto the screen
    pub fn display(&self, flow_box: &FlowBox) {
        let dim = flow_box.dim;

        let (block_size_x, block_size_y) = self.get_block_size(&dim);

        (0..dim.0 * dim.1).into_iter().for_each(|i| {
            let (x, y) = FlowBox::pos(&i, &dim);

            // Getting the correct color depending on display mode
            let color = match self.mode {
                DisplayMode::DensityColor => Color::new(
                    flow_box.density[i].x,
                    flow_box.density[i].y,
                    flow_box.density[i].z,
                    1.0,
                ),
                DisplayMode::DensityBlackWhite => {
                    let avg =
                        (flow_box.density[i].x + flow_box.density[i].y + flow_box.density[i].z)
                            / 3.0;
                    Color::new(avg, avg, avg, 1.0)
                }
                DisplayMode::VelocityBlackWhite => {
                    let vx = flow_box.vel_x[i].clamp(-100.0, 100.0) as f32;
                    let vy = flow_box.vel_y[i].clamp(-100.0, 100.0) as f32;
                    let m = Vec2::new(vx, vy).length_squared();
                    Color::new(m, m, m, 1.0)
                }
            };

            draw_rectangle(
                x as f32 * block_size_x,
                y as f32 * block_size_y,
                block_size_x,
                block_size_y,
                color,
            );
        });
        if self.flags & flags::DISPLAY_FPS != 0 {
            draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, WHITE);
        }
    }
}
