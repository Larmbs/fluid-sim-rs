use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::{DisplayMode, flags, FlowDisplay},
};

use macroquad::prelude::*;

#[macroquad::main("Fluid Sim")]
async fn main() {
    let mut flow_box = FlowBox::init(200, 100);

    let flow_display = FlowDisplay::init(
        DisplayMode::VelocityBlackWhite,
        flags::SHOW_VELOCITY_VECTORS,
    );

    loop {
        flow_box.step(1. / 30.);
        flow_display.display(&flow_box);
        next_frame().await;
    }
}
