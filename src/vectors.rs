use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn angle(self, rhs: Self) -> f32 {
        let dot = self * rhs;
        let mag = self.mag() * rhs.mag();
        (dot / mag).acos()
    }
}

impl Mul for Vec2 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_angle() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let angle = u.angle(v);
        let expected_angle = 0.225726; // Precomputed expected angle in radians
        let tolerance = 1e-6;
        assert!(
            (angle - expected_angle).abs() < tolerance,
            "Angle computation is incorrect"
        );
    }
}
