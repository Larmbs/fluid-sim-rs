//! Defines simple Vec2 object

/// 2d Vector struct
#[derive(Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    #[inline]
    pub fn zeroes() -> Vec2 {
        const VEC: Vec2 = Vec2 { x: 0., y: 0. };
        VEC
    }
    #[inline]
    pub fn ones() -> Vec2 {
        const VEC: Vec2 = Vec2 { x: 0., y: 0. };
        VEC
    }
    #[inline]
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
    #[inline]
    pub fn negate(&self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
    #[inline]
    pub fn interpolate(&self, other: &Vec2, t: f64) -> Vec2 {
        assert!(0. <= t && t <= 1., "t must be in range of 0..1 inclusive");
        Vec2 {
            x: self.x - (other.x - self.x) * t,
            y: self.y - (other.y - self.y) * t,
        }
    }
    #[inline]
    pub fn scale(&self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    #[inline]
    pub fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    #[inline]
    pub fn floor(&self) -> Vec2 {
        Vec2 {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
    #[inline]
    pub fn round(&self) -> Vec2 {
        Vec2 {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
    #[inline]
    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
    #[inline]
    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    #[inline]
    pub fn flip_y(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: -self.y,
        }
    }
    #[inline]
    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    #[inline]
    pub fn mag_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}
