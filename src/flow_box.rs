//! Defines fluid simulation logic

/// A box which holds a gird of fluid velocity vectors
pub struct FlowBox {
    pub dim: (usize, usize),

    pub vel_x: Vec<f64>,
    vel_x0: Vec<f64>,
    pub vel_y: Vec<f64>,
    vel_y0: Vec<f64>,

    visc: f64,
    diff: f64,

    diffuse_iters: usize,
    project_iters: usize,

    density0: Vec<f64>,
    pub density: Vec<f64>,

    iter: i128,
}
impl FlowBox {
    pub fn init(width: usize, height: usize) -> FlowBox {
        FlowBox {
            dim: (width, height),
            vel_x: vec![0.0; width * height],
            vel_x0: vec![0.0; width * height],
            vel_y: vec![0.0; width * height],
            vel_y0: vec![0.0; width * height],
            density0: vec![0.0; width * height],
            density: vec![0.0; width * height],
            visc: 0.00001,
            diff: 0.5,
            diffuse_iters: 2,
            project_iters: 1,
            iter: 0,
        }
    }
    /// Adds fluid density to a specified area
    pub fn add_fluid_density(&mut self, x: usize, y: usize, amount: f64) {
        self.density0[Self::index(x, y, &self.dim)] += amount;
        self.density0[Self::index(x + 1, y, &self.dim)] += amount;
        self.density0[Self::index(x, y + 1, &self.dim)] += amount;
        self.density0[Self::index(x + 1, y + 1, &self.dim)] += amount;
    }
    /// Adds fluid velocity (as horizontal an vertical components) to a specified area
    pub fn add_fluid_velocity(&mut self, x: usize, y: usize, vx: f64, vy: f64) {
        self.vel_x0[Self::index(x, y, &self.dim)] += vx;
        self.vel_x0[Self::index(x + 1, y, &self.dim)] += vx;
        self.vel_x0[Self::index(x, y + 1, &self.dim)] += vx;
        self.vel_x0[Self::index(x + 1, y + 1, &self.dim)] += vx;

        self.vel_y0[Self::index(x, y, &self.dim)] += vy;
        self.vel_y0[Self::index(x + 1, y, &self.dim)] += vy;
        self.vel_y0[Self::index(x, y + 1, &self.dim)] += vy;
        self.vel_y0[Self::index(x + 1, y + 1, &self.dim)] += vy;
    }
    /// Adds fluid velocity (as an angle and magnitude) to a specified area but
    pub fn add_fluid_velocity_angle_mag(&mut self, x: usize, y: usize, angle: f64, mag: f64) {
        let vx = angle.cos() * mag;
        let vy = angle.sin() * mag;
        self.add_fluid_velocity(x, y, vx, vy);
    }
}

impl FlowBox {
    // Steps the simulation forward given dt
    pub fn step(&mut self, dt: f64) {
        let center_x: usize = self.dim.0 / 2;
        let center_y: usize = self.dim.1 / 2;
        self.iter += 1;
        self.add_fluid_velocity_angle_mag(center_x, center_y, self.iter as f64 / 60., 20000.5);
        self.add_fluid_density(center_x, center_y, 0.5);

        Self::diffuse(
            1,
            &mut self.vel_x0,
            &self.vel_x,
            self.visc,
            dt,
            self.diffuse_iters,
            &self.dim,
        );
        Self::diffuse(
            2,
            &mut self.vel_y0,
            &self.vel_y,
            self.visc,
            dt,
            self.diffuse_iters,
            &self.dim,
        );

        Self::project(
            &mut self.vel_x0,
            &mut self.vel_y0,
            &mut self.vel_x,
            &mut self.vel_y,
            self.project_iters,
            &self.dim,
        );

        Self::advect(
            1,
            &mut self.vel_x,
            &self.vel_x0,
            &self.vel_x0,
            &self.vel_y0,
            dt,
            &self.dim,
        );
        Self::advect(
            2,
            &mut self.vel_y,
            &self.vel_y0,
            &self.vel_x0,
            &self.vel_y0,
            dt,
            &self.dim,
        );

        Self::project(
            &mut self.vel_x,
            &mut self.vel_y,
            &mut self.vel_x0,
            &mut self.vel_y0,
            self.project_iters,
            &self.dim,
        );

        Self::diffuse(
            0,
            &mut self.density0,
            &self.density,
            self.diff,
            dt,
            self.diffuse_iters,
            &self.dim,
        );
        Self::advect(
            0,
            &mut self.density,
            &self.density0,
            &self.vel_x,
            &self.vel_y,
            dt,
            &self.dim,
        );
    }
}

impl FlowBox {
    // Handles boundary conditions of the sim
    fn set_bound(bound: usize, values: &mut Vec<f64>, dim: &(usize, usize)) {
        // Deals with the top and bottom boundaries
        for x in 1..dim.0 - 1 {
            values[Self::index(x, 0, dim)] = if bound == 1 {
                -values[Self::index(x, 1, dim)]
            } else {
                values[Self::index(x, 1, dim)]
            };
            values[Self::index(x, dim.1 - 1, dim)] = if bound == 1 {
                -values[Self::index(x, dim.1 - 2, dim)]
            } else {
                values[Self::index(x, dim.1 - 2, dim)]
            };
        }

        // Deals with the side boundaries
        for y in 1..dim.1 - 1 {
            values[Self::index(0, y, dim)] = if bound == 2 {
                -values[Self::index(1, y, dim)]
            } else {
                values[Self::index(1, y, dim)]
            };
            values[Self::index(dim.0 - 1, y, dim)] = if bound == 2 {
                -values[Self::index(dim.0 - 2, y, dim)]
            } else {
                values[Self::index(dim.0 - 2, y, dim)]
            };
        }

        values[Self::index(0, 0, dim)] =
            0.5 * (values[Self::index(1, 0, dim)] + values[Self::index(0, 1, dim)]);

        values[Self::index(0, dim.1 - 1, dim)] =
            0.5 * (values[Self::index(1, dim.1 - 1, dim)] + values[Self::index(0, dim.1 - 2, dim)]);

        values[Self::index(dim.0 - 1, 0, dim)] =
            0.5 * (values[Self::index(dim.0 - 2, 0, dim)] + values[Self::index(dim.0 - 1, 1, dim)]);

        values[Self::index(dim.0 - 1, dim.1 - 1, dim)] = 0.5
            * (values[Self::index(dim.0 - 2, dim.1 - 1, dim)]
                + values[Self::index(dim.0 - 1, dim.1 - 2, dim)]);
    }
    /// Linear solver Gauss Seidel method
    fn lin_solve(
        b: usize,
        x: &mut Vec<f64>,
        x0: &Vec<f64>,
        a: f64,
        c: f64,
        iter: usize,
        dim: &(usize, usize),
    ) {
        let c_recip = 1.0 / c;

        for _ in 0..iter {
            for j in 1..dim.1 - 1 {
                for i in 1..dim.0 - 1 {
                    x[Self::index(i, j, dim)] = (x0[Self::index(i, j, dim)]
                        + a * (x[Self::index(i + 1, j, dim)]
                            + x[Self::index(i - 1, j, dim)]
                            + x[Self::index(i, j + 1, dim)]
                            + x[Self::index(i, j - 1, dim)]))
                        * c_recip;
                }
            }
            Self::set_bound(b, x, dim);
        }
    }
    /// Diffuses out values over a larger area
    fn diffuse(
        b: usize,
        x: &mut Vec<f64>,
        x0: &Vec<f64>,
        diff: f64,
        dt: f64,
        iter: usize,
        dim: &(usize, usize),
    ) {
        let a = dt * diff * (dim.0 - 2) as f64 * (dim.1 - 2) as f64;
        Self::lin_solve(b, x, x0, a, 1.0 + 4.0 * a, iter, dim);
    }
    /// Solves for divergence
    fn project(
        veloc_x: &mut Vec<f64>,
        veloc_y: &mut Vec<f64>,
        p: &mut Vec<f64>,
        div: &mut Vec<f64>,
        iter: usize,
        dim: &(usize, usize),
    ) {
        for j in 1..dim.1 - 1 {
            for i in 1..dim.0 - 1 {
                div[Self::index(i, j, dim)] = -0.5
                    * (veloc_x[Self::index(i + 1, j, dim)] - veloc_x[Self::index(i - 1, j, dim)]
                        + veloc_y[Self::index(i, j + 1, dim)]
                        - veloc_y[Self::index(i, j - 1, dim)])
                    / dim.0 as f64;
                p[Self::index(i, j, dim)] = 0.0;
            }
        }

        Self::set_bound(0, div, dim);
        Self::set_bound(0, p, dim);
        Self::lin_solve(0, p, div, 1.0, 1.9, iter, dim);

        for j in 1..dim.1 - 1 {
            for i in 1..dim.0 - 1 {
                veloc_x[Self::index(i, j, dim)] -=
                    0.5 * (p[Self::index(i + 1, j, dim)] - p[Self::index(i - 1, j, dim)]) as f64;
                veloc_y[Self::index(i, j, dim)] -=
                    0.5 * (p[Self::index(i, j + 1, dim)] - p[Self::index(i, j - 1, dim)]) as f64;
            }
        }

        Self::set_bound(1, veloc_x, dim);
        Self::set_bound(2, veloc_y, dim);
    }
    // Moves values along fluids direction of travel
    fn advect(
        b: usize,
        d: &mut Vec<f64>,
        d0: &Vec<f64>,
        veloc_x: &Vec<f64>,
        veloc_y: &Vec<f64>,
        dt: f64,
        dim: &(usize, usize),
    ) {
        let dtx = dt * (dim.0 - 2) as f64;
        let dty = dt * (dim.1 - 2) as f64;

        for j in 1..dim.1 - 1 {
            let j_float = j as f64;
            for i in 1..dim.0 - 1 {
                let i_float = i as f64;

                let tmp1 = dtx * veloc_x[Self::index(i, j, dim)];
                let tmp2 = dty * veloc_y[Self::index(i, j, dim)];

                let mut x = i_float - tmp1;
                let mut y = j_float - tmp2;

                if x < 0.5 {
                    x = 0.5;
                }
                if x > dim.0 as f64 + 0.5 {
                    x = dim.0 as f64 + 0.5;
                }
                let i0 = x.floor();
                let i1 = i0 + 1.0;

                if y < 0.5 {
                    y = 0.5;
                }
                if y > dim.1 as f64 + 0.5 {
                    y = dim.1 as f64 + 0.5;
                }
                let j0 = y.floor();
                let j1 = j0 + 1.0;

                let s1 = x - i0;
                let s0 = 1.0 - s1;
                let t1 = y - j0;
                let t0 = 1.0 - t1;

                let i0i = i0 as usize;
                let i1i = i1 as usize;
                let j0i = j0 as usize;
                let j1i = j1 as usize;

                d[Self::index(i, j, dim)] = s0
                    * (t0 * d0[Self::index(i0i, j0i, dim)] + t1 * d0[Self::index(i0i, j1i, dim)])
                    + s1 * (t0 * d0[Self::index(i1i, j0i, dim)]
                        + t1 * d0[Self::index(i1i, j1i, dim)]);
            }
        }
        Self::set_bound(b, d, dim);
    }
    // Returns index value for grid coord
    pub fn index(x: usize, y: usize, dim: &(usize, usize)) -> usize {
        (x + y * dim.0).clamp(0, dim.0 * dim.1 - 1)
    }
}
