//! Defines simple Vec2 object

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn zeroes() -> Vec2 {
        const VEC: Vec2 = Vec2 { x: 0., y: 0. };
        VEC
    }
    pub fn ones() -> Vec2 {
        const VEC: Vec2 = Vec2 { x: 0., y: 0. };
        VEC
    }
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn negate(&self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
    pub fn interpolate(&self, other: &Vec2, t: f64) -> Vec2 {
        assert!(0. <= t && t <= 1., "t must be in range of 0..1 inclusive");
        Vec2 {
            x: self.x - (other.x - self.x) * t,
            y: self.y - (other.y - self.y) * t,
        }
    }
    pub fn scale(&self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
