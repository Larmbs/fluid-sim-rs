//! Defines fluid simulation logic
use macroquad::color::Color;
use rayon::prelude::*;

/// Represents fluid simulation behavior
#[derive(PartialEq)]
pub struct FluidParams {
    pub viscosity: f64,
    pub diffusion_rate: f64,
    pub diffuse_iters: usize,
    pub project_iters: usize,
}
impl Default for FluidParams {
    fn default() -> Self {
        Self {
            viscosity: 0.00005,
            diffusion_rate: 0.00005,
            diffuse_iters: 3,
            project_iters: 4,
        }
    }
}

/// Represents what type of operation is being used on elements
#[derive(PartialEq, Eq)]
enum Bound {
    Neither,
    X,
    Y,
}

/// A box which holds a gird of fluid velocity vectors
pub struct FlowBox {
    pub dim: (usize, usize),

    pub vel_x: Vec<f64>,
    vel_x0: Vec<f64>,
    pub vel_y: Vec<f64>,
    vel_y0: Vec<f64>,

    pub red_density: Vec<f64>,
    pub green_density: Vec<f64>,
    pub blue_density: Vec<f64>,
    red_density0: Vec<f64>,
    green_density0: Vec<f64>,
    blue_density0: Vec<f64>,

    fluid_params: FluidParams,
}
impl FlowBox {
    /* Initializing */
    pub fn init(width: usize, height: usize) -> Self {
        FlowBox::init_with_params(width, height, FluidParams::default())
    }
    pub fn init_with_params(width: usize, height: usize, fluid_params: FluidParams) -> Self {
        FlowBox {
            dim: (width, height),
            vel_x: vec![0.0; width * height],
            vel_x0: vec![0.0; width * height],
            vel_y: vec![0.0; width * height],
            vel_y0: vec![0.0; width * height],
            red_density: vec![0.0; width * height],
            green_density: vec![0.0; width * height],
            blue_density: vec![0.0; width * height],
            red_density0: vec![0.0; width * height],
            green_density0: vec![0.0; width * height],
            blue_density0: vec![0.0; width * height],
            fluid_params,
        }
    }

    /* Interacting with Fluids */
    pub fn add_fluid_density(&mut self, x: usize, y: usize, color: Color) {
        let i = Self::index(
            &x.clamp(0, self.dim.0 - 1),
            &y.clamp(0, self.dim.1 - 1),
            &self.dim,
        );
        self.red_density[i] += color.r as f64;
        self.green_density[i] += color.g as f64;
        self.blue_density[i] += color.b as f64;
    }
    pub fn add_fluid_velocity(&mut self, x: usize, y: usize, vx: f64, vy: f64) {
        let i = Self::index(
            &x.clamp(0, self.dim.0 - 1),
            &y.clamp(0, self.dim.1 - 1),
            &self.dim,
        );
        self.vel_x0[i] += vx;
        self.vel_y0[i] += vy;
    }
    pub fn add_fluid_velocity_angle_mag(&mut self, x: usize, y: usize, angle: f64, mag: f64) {
        self.add_fluid_velocity(x, y, angle.cos() * mag, angle.sin() * mag);
    }
    pub fn scale_fluid_density(&mut self, mag: f64) {
        self.red_density.par_iter_mut().for_each(|d| *d *= mag);
        self.green_density.par_iter_mut().for_each(|d| *d *= mag);
        self.blue_density.par_iter_mut().for_each(|d| *d *= mag);
    }

    pub fn step(&mut self, dt: f64) {
        Self::diffuse(
            &Bound::X,
            &mut self.vel_x0,
            &self.vel_x,
            self.fluid_params.viscosity,
            dt,
            self.fluid_params.diffuse_iters,
            &self.dim,
        );
        Self::diffuse(
            &Bound::Y,
            &mut self.vel_y0,
            &self.vel_y,
            self.fluid_params.viscosity,
            dt,
            self.fluid_params.diffuse_iters,
            &self.dim,
        );

        Self::project(
            &mut self.vel_x0,
            &mut self.vel_y0,
            &mut self.vel_x,
            &mut self.vel_y,
            self.fluid_params.project_iters,
            &self.dim,
        );

        Self::advect(
            &Bound::X,
            &mut self.vel_x,
            &self.vel_x0,
            &self.vel_x0,
            &self.vel_y0,
            dt,
            &self.dim,
        );
        Self::advect(
            &Bound::Y,
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
            self.fluid_params.project_iters,
            &self.dim,
        );

        Self::diffuse(
            &Bound::Neither,
            &mut self.red_density0,
            &self.red_density,
            self.fluid_params.diffusion_rate,
            dt,
            self.fluid_params.diffuse_iters,
            &self.dim,
        );
        Self::diffuse(
            &Bound::Neither,
            &mut self.green_density0,
            &self.green_density,
            self.fluid_params.diffusion_rate,
            dt,
            self.fluid_params.diffuse_iters,
            &self.dim,
        );
        Self::diffuse(
            &Bound::Neither,
            &mut self.blue_density0,
            &self.blue_density,
            self.fluid_params.diffusion_rate,
            dt,
            self.fluid_params.diffuse_iters,
            &self.dim,
        );
        Self::advect(
            &Bound::Neither,
            &mut self.red_density,
            &self.red_density0,
            &self.vel_x,
            &self.vel_y,
            dt,
            &self.dim,
        );
        Self::advect(
            &Bound::Neither,
            &mut self.green_density,
            &self.green_density0,
            &self.vel_x,
            &self.vel_y,
            dt,
            &self.dim,
        );
        Self::advect(
            &Bound::Neither,
            &mut self.blue_density,
            &self.blue_density0,
            &self.vel_x,
            &self.vel_y,
            dt,
            &self.dim,
        );
    }

    // Handles boundary conditions of the sim
    fn set_bound(bound: &Bound, vals: &mut [f64], dim: &(usize, usize)) {
        // Deals with the top and bottom boundaries
        let vals_clone = vals.to_vec();
        // let (row1, rest) = vals.split_at_mut(dim.0);
        // let (_, row_last) = rest.split_at_mut(rest.len() - dim.0);

        // row1.par_iter_mut().zip(row_last.par_iter_mut())
        let dir = if bound == &Bound::X { -1.0 } else { 1.0 };
        for x in 1..dim.0 - 1 {
            vals[Self::index(&x, &0, dim)] = dir * vals_clone[Self::index(&x, &1, dim)];
            vals[Self::index(&x, &(dim.1 - 1), dim)] =
                dir * vals_clone[Self::index(&x, &(dim.1 - 2), dim)];
        }

        // Deals with the side boundaries
        let dir = if bound == &Bound::Y { -1.0 } else { 1.0 };
        vals.par_chunks_mut(dim.0)
            .enumerate()
            .filter(|(y, _)| (1..dim.1 - 1).contains(&y))
            .for_each(|(y, row)| {
                row[0] = dir * vals_clone[Self::index(&1, &y, dim)];
                row[dim.0 - 1] = dir * vals_clone[Self::index(&(dim.0 - 2), &y, dim)];
            });

        vals[Self::index(&0, &0, dim)] =
            0.5 * (vals[Self::index(&1, &0, dim)] + vals[Self::index(&0, &1, dim)]);

        vals[Self::index(&0, &(dim.1 - 1), dim)] = 0.5
            * (vals[Self::index(&1, &(dim.1 - 1), dim)] + vals[Self::index(&0, &(dim.1 - 2), dim)]);

        vals[Self::index(&(dim.0 - 1), &0, dim)] = 0.5
            * (vals[Self::index(&(dim.0 - 2), &0, dim)] + vals[Self::index(&(dim.0 - 1), &1, dim)]);

        vals[Self::index(&(dim.0 - 1), &(dim.1 - 1), dim)] = 0.5
            * (vals[Self::index(&(dim.0 - 2), &(dim.1 - 1), dim)]
                + vals[Self::index(&(dim.0 - 1), &(dim.1 - 2), dim)]);
    }
    /// Linear solver Gauss Seidel method
    fn lin_solve(
        bound: &Bound,
        vals: &mut [f64],
        vals0: &[f64],
        a: f64,
        c: f64,
        iters: usize,
        dim: &(usize, usize),
    ) {
        let c_recip = 1.0 / c;

        for _ in 0..iters {
            let clone_vals = vals.to_vec();

            vals.par_iter_mut().enumerate().for_each(|(i, v)| {
                let (x, y) = Self::pos(&i, dim);
                if (1..dim.0 - 1).contains(&x) && (1..dim.1 - 1).contains(&y) {
                    *v = (vals0[i]
                        + a * (clone_vals[Self::index(&(x + 1), &y, dim)]
                            + clone_vals[Self::index(&(x - 1), &y, dim)]
                            + clone_vals[Self::index(&x, &(y + 1), dim)]
                            + clone_vals[Self::index(&x, &(y - 1), dim)]))
                        * c_recip;
                }
            });

            Self::set_bound(bound, vals, dim);
        }
    }
    /// Diffuses out values over a larger area
    fn diffuse(
        b: &Bound,
        vals: &mut [f64],
        vals0: &[f64],
        diff: f64,
        dt: f64,
        iters: usize,
        dim: &(usize, usize),
    ) {
        let a = dt * diff * 10000.0;
        Self::lin_solve(b, vals, vals0, a, 1.0 + 4.0 * a, iters, dim);
    }
    /// Solves for divergence
    fn project(
        vel_x: &mut [f64],
        vel_y: &mut [f64],
        p: &mut [f64],
        div: &mut [f64],
        iters: usize,
        dim: &(usize, usize),
    ) {
        div.par_iter_mut()
            .zip(p.par_iter_mut())
            .enumerate()
            .for_each(|(i, (v, pv))| {
                let (x, y) = Self::pos(&i, dim);

                if (1..dim.0 - 1).contains(&x) && (1..dim.1 - 1).contains(&y) {
                    *v = -0.5
                        * (vel_x[Self::index(&(x + 1), &&y, dim)]
                            - vel_x[Self::index(&(x - 1), &y, dim)]
                            + vel_y[Self::index(&x, &(y + 1), dim)]
                            - vel_y[Self::index(&x, &(y - 1), dim)]);
                }

                *pv = 0.0;
            });

        Self::set_bound(&Bound::Neither, div, dim);
        Self::set_bound(&Bound::Neither, p, dim);
        Self::lin_solve(&Bound::Neither, p, div, 1.0, 6.0, iters, dim);

        vel_x
            .par_iter_mut()
            .zip(vel_y.par_iter_mut())
            .enumerate()
            .for_each(|(i, (vx, vy))| {
                let (x, y) = Self::pos(&i, dim);

                if (1..dim.0 - 1).contains(&x) && (1..dim.1 - 1).contains(&y) {
                    *vx -= 0.5
                        * (p[Self::index(&(x + 1), &y, dim)] - p[Self::index(&(x - 1), &y, dim)]);
                    *vy -= 0.5
                        * (p[Self::index(&x, &(y + 1), dim)] - p[Self::index(&x, &(y - 1), dim)]);
                }
            });

        Self::set_bound(&Bound::X, vel_x, dim);
        Self::set_bound(&Bound::Y, vel_y, dim);
    }
    // Moves values along fluids direction of travel
    fn advect(
        bound: &Bound,
        vals: &mut [f64],
        vals0: &[f64],
        vel_x: &[f64],
        vel_y: &[f64],
        dt: f64,
        dim: &(usize, usize),
    ) {
        let dtx = dt * 100.0;
        let dty = dt * 100.0;

        vals.par_iter_mut().enumerate().for_each(|(ix, v)| {
            let (i, j) = Self::pos(&ix, dim);

            let j_float = j as f64;
            let i_float = i as f64;

            let tmp1 = dtx * vel_x[ix];
            let tmp2 = dty * vel_y[ix];

            let mut x = i_float - tmp1;
            let mut y = j_float - tmp2;

            x = x.clamp(0.5, dim.0 as f64 - 0.5);

            let i0 = x.floor();
            let i1 = i0 + 1.0;

            y = y.clamp(0.5, dim.1 as f64 - 0.5);

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

            *v = s0
                * (t0 * vals0[Self::index(&i0i, &j0i, dim).clamp(0, dim.0 * dim.1 - 1)]
                    + t1 * vals0[Self::index(&i0i, &j1i, dim).clamp(0, dim.0 * dim.1 - 1)])
                + s1 * (t0 * vals0[Self::index(&i1i, &j0i, dim).clamp(0, dim.0 * dim.1 - 1)]
                    + t1 * vals0[Self::index(&i1i, &j1i, dim).clamp(0, dim.0 * dim.1 - 1)]);
        });

        Self::set_bound(bound, vals, dim);
    }
    // Returns index value for grid coord
    #[inline]
    pub fn index(x: &usize, y: &usize, dim: &(usize, usize)) -> usize {
        x + y * dim.0
    }
    #[inline]
    pub fn pos(i: &usize, dim: &(usize, usize)) -> (usize, usize) {
        (i % dim.0, i / dim.0)
    }
}
