pub struct Particle {
    x: f64,
    y: f64
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Particle{x, y}
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}