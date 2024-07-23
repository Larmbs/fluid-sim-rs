//! Defines fluid simulation logic
#![allow(unused)]

use super::vec2::Vec2;

/// Module defines private defaults for values
mod default {
    pub const APPLY_BORDER: bool = true;
    pub const DIVERGENCE_ITERATIONS: usize = 50;
    pub const DIFFUSION_RATE: f64 = 1.;
}

pub struct FlowBox {
    width: usize,
    height: usize,

    // 2d grid of fluid velocities
    vector_field: Vec<Vec2>,
    // 2d grid of obstacles in sim
    obstacle_field: Vec<bool>,

    /* Some settings for fluid behavior */
    viscosity: f64,
    apply_border: bool,
    divergence_iters: usize,
    diffuse_rate: f64,
}
impl FlowBox {
    pub fn init(width: usize, height: usize, viscosity: f64) -> FlowBox {
        FlowBox {
            width,
            height,
            vector_field: (0..(width * height))
                .into_iter()
                .map(|_| Vec2::zeroes())
                .collect(),
            obstacle_field: vec![],
            viscosity,
            apply_border: default::APPLY_BORDER,
            divergence_iters: default::DIVERGENCE_ITERATIONS,
            diffuse_rate: default::DIFFUSION_RATE,
        }
    }
    pub fn set_obstacles(&mut self, obstacles: Vec<bool>) {
        assert_eq!(
            obstacles.len(),
            self.vector_field.len(),
            "Obstacle grid provided has incompatible dimensions"
        );
        self.obstacle_field = obstacles;
    }
}

impl FlowBox {
    // Steps simulation forward given dt
    pub fn step(&mut self, dt: f64) {
        todo!()
    }
}

impl FlowBox {
    fn apply_advection(&mut self, dt: f64) {
        todo!()
    }
    fn apply_diffusion(&mut self, dt: f64) {
        // Smoothing filter with tuples (dx, dy, weight)
        const GAUSSIAN_FILTER: [(isize, isize, usize); 9] = [
            (1, 1, 1),
            (1, -1, 1),
            (-1, 1, 1),
            (-1, -1, 1),
            (0, 1, 2),
            (0, -1, 2),
            (1, 0, 2),
            (-1, 0, 2),
            (0, 0, 4),
        ];

        for i in 0..self.vector_field.len() {
            let x = i % self.width;
            let y = i / self.width;

            let mut sum_x = 0.;
            let mut sum_y = 0.;
            let mut weight_sum = 0;
            for (dx, dy, weight) in GAUSSIAN_FILTER {
                let sample_x = x as isize + dx;
                let sample_y = y as isize + dy;

                if 0 <= sample_x
                    && sample_x < self.width as isize
                    && 0 <= sample_y
                    && sample_y <= self.height as isize
                {
                    weight_sum += weight;
                    let sample =
                        &self.vector_field[sample_x as usize + sample_y as usize * self.width];
                    sum_x += sample.x * weight as f64;
                    sum_y += sample.y * weight as f64;
                }
            }
            let current = &self.vector_field[x + y * self.width];
            let next = Vec2::new(sum_x, sum_y).scale(1. / weight_sum as f64);
            let t = self.diffuse_rate * dt / 2.;

            let new = current.interpolate(&next, t);
            self.vector_field[x + y * self.width] = new;
        }
    }
    fn remove_divergence(&mut self, dt: f64) {
        // Attempts to stop fluid from leaving areas of low pressure
        todo!()
    }
}
