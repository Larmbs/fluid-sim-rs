use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::{DisplayMode, FlowDisplay},
};

use macroquad::prelude::*;

#[macroquad::main("Fluid Sim")]
async fn main() {
    let mut flow_box = FlowBox::init(100, 50);

    let flow_display = FlowDisplay::init(DisplayMode::VelocityBlackWhite);

    loop {
        flow_box.step(1. / 30.);
        flow_display.display(&flow_box);
        next_frame().await;
    }
}
