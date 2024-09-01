use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::{flags, DisplayMode, FlowDisplay},
};

use macroquad::prelude::*;

#[macroquad::main("Fluid Sim")]
async fn main() {
    const WIDTH: usize = 200;
    const HEIGHT: usize = 100;

    let mut flow_box = FlowBox::init(WIDTH, HEIGHT);

    let flow_display = FlowDisplay::init(DisplayMode::VelocityBlackWhite, flags::NONE);

    const CENTER_X: usize = WIDTH / 2;
    const CENTER_Y: usize = HEIGHT / 2;

    let mut iter: u128 = 0;
    loop {
        flow_box.scale_fluid_density(0.9);

        // let pos = flow_display.get_mouse_cord(&flow_box);
        // let angle = flow_display.get_mouse_delta_angle();

        // Interacting with fluid
        flow_box.add_fluid_velocity_angle_mag(CENTER_X, CENTER_Y, iter as f64 / 60., 90000.0);
        flow_box.add_fluid_density(CENTER_X, CENTER_Y, 15.0);

        // Simulating and drawing
        flow_box.step(1. / 30.);
        flow_display.display(&flow_box);

        // Next frame
        iter = iter.wrapping_add(1);
        next_frame().await;
    }
}
