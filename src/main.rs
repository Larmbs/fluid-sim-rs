use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::{DisplayMode, FlowDisplay},
};

use macroquad::prelude::*;

#[macroquad::main("Fluid Sim")]
async fn main() {
    let mut flow_box = FlowBox::init(100, 50, 0.5);

    let display_mode = DisplayMode::VelocityBlackWhite;

    loop {
        flow_box.step(1.);
        FlowDisplay::display(&flow_box, &display_mode);
        next_frame().await;
    }
}
