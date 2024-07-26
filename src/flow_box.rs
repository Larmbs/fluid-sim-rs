//! Defines fluid simulation logic
#![allow(unused)]

use super::vec2::Vec2;

/// Module defines private defaults for values
mod default {
    pub const APPLY_BORDER: bool = true;
    pub const DIVERGENCE_ITERATIONS: usize = 50;
    pub const DIFFUSION_RATE: f64 = 1.;
}

/// An object that holds a sandbox containing fluid
pub struct FlowBox {
    pub width: usize,
    pub height: usize,

    // 2d grid of fluid velocities
    pub vec_field: Vec<Vec2>,
    // 2d grid of obstacles in sim
    obstacle_field: Vec<bool>,

    /* Some settings for fluid behavior */
    gravity: f64,
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
            vec_field: (0..(width * height))
                .into_iter()
                .map(|_| Vec2::zeroes())
                .collect(),
            obstacle_field: vec![],
            gravity: -9.8,
            viscosity,
            apply_border: default::APPLY_BORDER,
            divergence_iters: default::DIVERGENCE_ITERATIONS,
            diffuse_rate: default::DIFFUSION_RATE,
        }
    }
    pub fn set_obstacles(&mut self, obstacles: Vec<bool>) {
        assert_eq!(
            obstacles.len(),
            self.vec_field.len(),
            "Obstacle grid provided has incompatible dimensions"
        );
        self.obstacle_field = obstacles;
    }
}

impl FlowBox {
    // Steps simulation forward given dt
    pub fn step(&mut self, dt: f64) {
        self.vec_field[self.width / 2 + self.height / 2 * self.width].x = 0.5;
        self.apply_diffusion(dt);
        //self.apply_advection(dt);

        for _ in 0..self.divergence_iters {
            //self.remove_divergence(dt);
        }
    }
}

impl FlowBox {
    fn index_vec_field(&self, index: &Vec2) -> &Vec2 {
        assert_eq!(
            index.x.fract(),
            0.0,
            "Index provided has a fractional component and cannot be used"
        );
        assert_eq!(
            index.y.fract(),
            0.0,
            "Index provided has a fractional component and cannot be used"
        );
        &self.vec_field[index.x as usize + index.y as usize * self.width]
    }
    /// Moving velocities in direction of their velocity
    fn apply_advection(&mut self, dt: f64) {
        let mut new_vec_field: Vec<Vec2> = Vec::with_capacity(self.vec_field.len());

        for i in 0..self.vec_field.len() {
            let x = i % self.width;
            let y = i / self.width;

            let current_pos = Vec2::new(x as f64, y as f64);
            let back_pos = current_pos.sub(&self.vec_field[i].scale(dt));
            let back_pos_i = back_pos.floor(); // Backward Position Index
            let frac_diff = back_pos.sub(&back_pos_i); // Fractional Difference

            let z3: Vec2;

            // Checking for bounds
            let z3 = if 0. <= back_pos_i.x
                && back_pos_i.x < self.width as f64
                && 0. <= back_pos_i.y
                && back_pos_i.y < self.height as f64
            {
                let z1 = Vec2::new(back_pos_i.x, back_pos_i.y)
                    .interpolate(&Vec2::new(back_pos_i.x + 1., back_pos_i.y), frac_diff.x);
                let z2 = Vec2::new(back_pos_i.x, back_pos_i.y + 1.).interpolate(
                    &Vec2::new(back_pos_i.x + 1., back_pos_i.y + 1.),
                    frac_diff.x,
                );

                z1.interpolate(&z2, frac_diff.y)
            } else {
                let mut z3 = self.vec_field[i].clone();
                if (x == 0 || x == self.width - 1) {
                    z3.x *= -1.;
                }
                if (y == 0 || y == self.height - 1) {
                    z3.y *= -1.;
                }
                z3
            };

            new_vec_field.push(z3);
        }

        self.vec_field = new_vec_field;
    }
    /// Spreads out pressure and velocity over area (diffusing)
    fn apply_diffusion(&mut self, dt: f64) {
        let mut new_vec_field: Vec<Vec2> = vec![Vec2::zeroes(); self.vec_field.len()];

        // Smoothing filter with tuples (dx, dy, weight)
        const DIFFUSION_FILTER: [(isize, isize, usize); 5] = [
            (0, 1, 1),
            (0, -1, 1),
            (-1, 0, 1),
            (1, 0, 1),
            (0, 0, 1), // Include the center point
        ];

        for i in 0..self.vec_field.len() {
            let x = i % self.width;
            let y = i / self.width;

            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut weight_sum = 0;

            for (dx, dy, weight) in DIFFUSION_FILTER {
                let sample_x = x as isize + dx;
                let sample_y = y as isize + dy;

                // Checking whether in bounds
                if sample_x >= 0
                    && sample_x < self.width as isize
                    && sample_y >= 0
                    && sample_y < self.height as isize
                {
                    weight_sum += weight;
                    let sample =
                        &self.vec_field[sample_x as usize + sample_y as usize * self.width];
                    sum_x += sample.x * weight as f64;
                    sum_y += sample.y * weight as f64;
                }
            }

            if weight_sum > 0 {
                let current = &self.vec_field[x + y * self.width];
                let next = Vec2::new(sum_x, sum_y).scale(1.0 / weight_sum as f64);
                let t = self.diffuse_rate * dt / 2.0;

                let new = current.interpolate(&next, t);
                new_vec_field[i] = new;
            } else {
                new_vec_field[i] = self.vec_field[i].clone();
            }
        }

        self.vec_field = new_vec_field;
    }

    // Attempts to stop fluids from leaving areas of low pressure
    fn remove_divergence(&mut self, dt: f64) {
        for i in 0..self.vec_field.len() {
            let x = i % self.width;
            let y = i / self.width;

            if 1 <= x && x < self.width - 1 && 1 <= y && y < self.height - 1 {
                let dot1 = self.vec_field[(x + 1) + (y + 1) * self.width]
                    .add(&self.vec_field[(x - 1) + (y - 1) * self.width])
                    .dot(&Vec2::ones());
                let dot2 = self.vec_field[(x + 1) + (y - 1) * self.width]
                    .add(&self.vec_field[(x - 1) + (y + 1) * self.width])
                    .dot(&Vec2::ones().flip_y());

                let grad = self.vec_field[(x) + (y + 1) * self.width]
                    .sub(&self.vec_field[(x) + (y - 1) * self.width])
                    .add(&self.vec_field[(x + 1) + (y) * self.width])
                    .sub(&self.vec_field[(x - 1) + (y) * self.width]);

                let res = grad.add(&Vec2::new(dot1 + dot2, dot1 - dot2));

                self.vec_field[i] = res.scale(dt / self.divergence_iters as f64);
            }
        }
    }
    fn add_gravity(&mut self, dt: f64) {
        for i in 0..self.vec_field.len() {
            self.vec_field[i].y = self.vec_field[i].y + dt * self.gravity
        }
    }
}
