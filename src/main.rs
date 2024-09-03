use fluid_sim_rs::{
    flow_box::FlowBox,
    flow_display::FlowDisplay,
};

use miniquad::*;

const WIDTH: usize = 150;
const HEIGHT: usize = 75;
const CELLS: usize = WIDTH * HEIGHT;

fn main() {
    let flow_box = FlowBox::<CELLS>::init(WIDTH, HEIGHT);
    miniquad::start(conf::Conf::default(), move || Box::new(FlowDisplay::new(flow_box)));
}
