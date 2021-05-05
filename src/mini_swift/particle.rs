pub struct Particle {
    x: f64,
    y: f64,
    pub h: f64,
    pub added_to_del_tess: u16
}

impl Particle {
    pub fn new(x: f64, y: f64, h: f64) -> Self {
        Particle{x, y, h, added_to_del_tess: 0}
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}