mod voronoi2d;
mod delaunay2d;

pub use delaunay2d::DelaunayTriangulation2D;
pub use voronoi2d::VoronoiGrid2D;
use std::ops;


/// A simple point in 2D space
#[derive(Debug)]
struct Vertex2D {
    pub(super) x: f64,
    pub(super) y: f64
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