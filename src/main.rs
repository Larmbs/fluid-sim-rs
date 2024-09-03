use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::{flags, DisplayMode, FlowDisplay},
};

use macroquad::{color::hsl_to_rgb, prelude::*};

const WIDTH: usize = 150;
const HEIGHT: usize = 75;
const CELLS: usize = WIDTH * HEIGHT;

#[macroquad::main("Fluid Sim")]
async fn main() {
    let mut flow_box = FlowBox::<CELLS>::init(WIDTH, HEIGHT);

    let mut flow_display = FlowDisplay::init(DisplayMode::DensityColor, flags::DISPLAY_FPS);

    let mut iter: u128 = 0;
    loop {
        let pos = flow_display.get_mouse_cord(&flow_box.dim);
        let angle = flow_display.get_mouse_mov_dir();

        flow_box.add_fluid_velocity_angle_mag(pos.0, pos.1, angle, 90000.0);
        flow_box.add_fluid_density(
            pos.0,
            pos.1,
            hsl_to_rgb(((iter as f32) / 128.0) % 1.0, 25.0, 0.5).into(),
        );

        // Simulating and drawing
        flow_box.step(1.0 / 30.0);
        flow_display.display(&flow_box);

        // Next frame
        iter = iter.wrapping_add(1);
        next_frame().await;
    }
}
