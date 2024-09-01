//! This is a simple Fluid Sim inspired by these articles
//! https://matthias-research.github.io/pages/tenMinutePhysics/17-fluidSim.pdf
//! https://www.mikeash.com/pyblog/fluid-simulation-for-dummies.html

/// A grid holding velocities and density of particles within fluid
pub mod flow_box;
/// An object capable of displaying a FlowBox with different modes and settings
/// Also offers simple and convenient functions to interact with fluid
pub mod flow_display;
