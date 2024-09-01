//! Defines object capable of drawing a FlowBox

use std::f32::consts::PI;

use super::flow_box::FlowBox;
use lazy_static::lazy_static;
use macroquad::prelude::*;

/// Different viewing modes for FlowBox
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
}

lazy_static! {
    static ref SPEED_COLORS: [Color; 3] = [
        Color::from_hex(0x80ff2b),
        Color::from_hex(0xffd429),
        Color::from_hex(0xff3729)
    ];
}

// /// Linear interpolates between two colors s 0.0-1.0
// fn lerp_colors(color1: &Color, color2: &Color, s: f32) -> Color {
//     Color::from_vec(color1.to_vec().lerp(color2.to_vec(), s))
// }

// /// Linear interpolates between three colors s 0.0-2.0
// fn lerp_3_colors(color1: &Color, color2: &Color, color3: &Color, s: f32) -> Color {
//     match s {
//         0.0..=1.0 => lerp_colors(color1, color2, s),
//         1.0..=2.0 => lerp_colors(color2, color3, s - 1.0),
//         _ => *color3,
//     }
// }

/// Displays a FlowBox
pub struct FlowDisplay {
    mode: DisplayMode,
    flags: u8,
    last_d_mouse_angle: f32,
}
impl FlowDisplay {
    pub fn init(mode: DisplayMode, flags: u8) -> Self {
        Self {
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
    /// Returns the last direction mouse was moving in the window
    pub fn get_mouse_delta_angle(&mut self) -> f32 {
        let mouse_delta = mouse_delta_position();
        let angle = -mouse_delta.angle_between(Vec2::from_angle(0.0)) + PI;
        if angle.is_finite() {
            self.last_d_mouse_angle = angle;
            angle
        } else {
            self.last_d_mouse_angle
        }
    }
    /// Displays fluid onto the screen
    pub fn display(&self, flow_box: &FlowBox) {
        let dim = flow_box.dim;

        let block_size = (screen_width() / dim.0 as f32).min(screen_height() / dim.1 as f32);
        //let half_block_size = block_size / 2.0;

        for x in 0..dim.0 {
            let screen_x = x as f32 * block_size;
            for y in 0..dim.1 {
                let screen_y = y as f32 * block_size;

                let i = FlowBox::index(&x, &y, &dim);

                // Getting the correct color depending on display mode
                let color = match self.mode {
                    DisplayMode::DensityColor => Color {
                        r: flow_box.red_density[i]
                            as f32,
                        g: flow_box.green_density[i]
                            as f32,
                        b: flow_box.blue_density[i]
                            as f32,
                        a: 1.0,
                    },
                    DisplayMode::DensityBlackWhite => {
                        let avg = ((flow_box.red_density[i]
                            + flow_box.red_density[i]
                            + flow_box.red_density[i])
                            / 3.0) as f32;
                        Color {
                            r: avg,
                            g: avg,
                            b: avg,
                            a: 1.0,
                        }
                    }
                    DisplayMode::VelocityBlackWhite => {
                        let vx = flow_box.vel_x[i].clamp(-100.0, 100.0) as f32;
                        let vy = flow_box.vel_y[i].clamp(-100.0, 100.0) as f32;
                        let mag = Vec2::new(vx, vy).length_squared();
                        Color {
                            r: mag,
                            g: mag,
                            b: mag,
                            a: 1.0,
                        }
                    },
                };

                draw_rectangle(screen_x, screen_y, block_size, block_size, color);

                if self.flags & flags::SHOW_VELOCITY_VECTORS != 0 {
                    // Needs a rework

                    // let x1 = screen_x + half_block_size;
                    // let y1 = screen_y + half_block_size;

                    // let vx = (flow_box.vel_x[i] * 80.0).clamp(-6.0, 6.0)
                    //     as f32;
                    // let vy = (flow_box.vel_y[i] * 80.0).clamp(-6.0, 6.0)
                    //     as f32;
                    // let mag_sq = vx.powi(2) + vy.powi(2);
                    // let scalar = mag_sq / 36.0;

                    // let color =
                    //     lerp_3_colors(&SPEED_COLORS[0], &SPEED_COLORS[1], &SPEED_COLORS[2], scalar);

                    // draw_line(x1, y1, x1 + vx, y1 + vy, 1.0, color)
                }
            }
        }
        if self.flags & flags::DISPLAY_FPS != 0 {
            draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, WHITE);
        }
    }
}
