use super::{Vertex2D, oriented_volume_2d, centroid_2d};


pub struct Triangle2D {
    a: Vertex2D,
    b: Vertex2D,
    c: Vertex2D
}

impl Triangle2D {
    pub fn new(a: Vertex2D, b: Vertex2D, c: Vertex2D) -> Triangle2D {
        Triangle2D{a, b, c}
    }

    pub fn area(&self) -> f64 {
        oriented_volume_2d(self.a.x, self.a.y,
                           self.b.x, self.b.y,
                           self.c.x, self.c.y)
    }

    pub fn centroid(&self) -> Vertex2D {
        centroid_2d(self.a.x, self.a.y, self.b.x, self.b.y, self.c.x, self.c.y)
    }
}