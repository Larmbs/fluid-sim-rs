use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::{flags, DisplayMode, FlowDisplay},
};

use macroquad::{color::hsl_to_rgb, prelude::*};

#[macroquad::main("Fluid Sim")]
async fn main() {
    const WIDTH: usize = 150;
    const HEIGHT: usize = 75;

    let mut flow_box = FlowBox::init(WIDTH, HEIGHT);

    let flow_display = FlowDisplay::init(DisplayMode::VelocityBlackWhite, flags::NONE);

    const CENTER_X: usize = WIDTH / 2;
    const CENTER_Y: usize = HEIGHT / 2;

    let mut iter: u128 = 0;
    loop {
        //flow_box.scale_fluid_density(0.999);

        // let pos = flow_display.get_mouse_cord(&flow_box);
        // let angle = flow_display.get_mouse_delta_angle();
        let angle = iter as f64 / 60.;
        // Interacting with fluid
        flow_box.add_fluid_velocity_angle_mag(CENTER_X, CENTER_Y, angle, 90000.0);
        flow_box.add_fluid_density(CENTER_X, CENTER_Y, hsl_to_rgb(angle as f32 % 1.0, 25.0, 0.5));

        // Simulating and drawing
        flow_box.step(1. / 30.);
        flow_display.display(&flow_box);

        // Next frame
        iter = iter.wrapping_add(1);
        next_frame().await;
    }
}
