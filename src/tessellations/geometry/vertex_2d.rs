use std::ops;

/// A simple point in 2D space
#[derive(Debug, Copy, Clone)]
pub struct Vertex2D {
    pub x: f64,
    pub y: f64
}

impl Default for Vertex2D {
    fn default() -> Vertex2D {
        Vertex2D{x: std::f64::NAN, y: std::f64::NAN}
    }
}

impl ops::Add<Vertex2D> for Vertex2D {
    type Output = Vertex2D;

    fn add(self, rhs: Vertex2D) -> Vertex2D {
        Vertex2D{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl ops::AddAssign for Vertex2D {
    fn add_assign(&mut self, rhs: Vertex2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vertex2D> for Vertex2D {
    type Output = Vertex2D;

    fn sub(self, rhs: Vertex2D) -> Vertex2D {
        Vertex2D{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl ops::Mul<f64> for Vertex2D {
    type Output = Vertex2D;

    fn mul(self, rhs: f64) -> Vertex2D {
        Vertex2D{x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Mul<Vertex2D> for f64 {
    type Output = Vertex2D;

    fn mul(self, rhs: Vertex2D) -> Vertex2D {
        Vertex2D{x: rhs.x * self, y: rhs.y * self }
    }
}

impl ops::DivAssign<f64> for Vertex2D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Div<f64> for Vertex2D {
    type Output = Vertex2D;

    fn div(self, rhs: f64) -> Vertex2D {
        Vertex2D{x: self.x / rhs, y: self.y / rhs }
    }
}

impl Vertex2D {
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.x*self.x + self.y*self.y)
    }
}