
#[derive(Debug, Default)]
pub struct SimulationDomain2D {
    anchor: [f64; 2],
    sides: [f64; 2],
}

impl SimulationDomain2D {
    pub fn new(a: [f64; 2], s: [f64; 2]) -> SimulationDomain2D {
        SimulationDomain2D{anchor: a, sides: s}
    }

    pub fn anchor(&self) -> &[f64; 2] {
        &self.anchor
    }

    pub fn sides(&self) -> &[f64; 2] {
        &self.sides
    }
}