use crate::mini_swift::particle::Particle;
use crate::tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};

#[derive(Default)]
pub struct Cell {
    anchor: [f64; 2],
    sides: [f64; 2],
    particles: Option<Vec<Particle>>,
    progeny: Option<[Box<Cell>; 4]>,
    del_tess: Option<DelaunayTriangulation2D>,
    vor_tess: Option<VoronoiGrid2D>,
}

impl Cell {
    pub fn from_dimensions(anchor: [f64; 2], sides: [f64; 2]) -> Self {
        Cell {
            anchor,
            sides,
            ..Cell::default()
        }
    }

    pub fn add_particles(&mut self, x_vals: &[f64], y_vals: &[f64]) {
        for (&x, &y) in x_vals.iter().zip(y_vals.iter()) {
            self.add_particle(x, y);
        }
    }

    fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.as_mut().unwrap().push(Particle::new(x, y));
    }

    pub fn split(&mut self) {
        assert!(self.progeny.is_none() && self.particles.is_some(),
                "Trying to split a cell which is not a leaf!");

        // create progeny
        let anchor = self.anchor;
        let half_sides = [self.sides[0] / 2., self.sides[1] / 1.];
        let mut progeny = [
            Box::new(Cell::from_dimensions(anchor, half_sides)),
            Box::new(Cell::from_dimensions([anchor[0] + half_sides[0], anchor[1]], half_sides)),
            Box::new(Cell::from_dimensions([anchor[0], anchor[1] + half_sides[1]], half_sides)),
            Box::new(Cell::from_dimensions([anchor[0] + half_sides[0], anchor[1] + half_sides[1]], half_sides))
        ];
        // divide particles over progeny
        for particle in self.particles.as_ref().unwrap() {
            let pid = (particle.x() >= anchor[0] + half_sides[0]) as u8
                + 2 * ((particle.y() >= anchor[1] + half_sides[1]) as u8);
            progeny[pid as usize].add_particle(particle.x(), particle.y());
        }

        self.progeny = Some(progeny);
        self.particles = None;
    }
}