use crate::mini_swift::particle::Particle;
use crate::simulation_domain_2d::SimulationDomain2D;
use crate::tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};

mod hydro_iact_density;
mod hydro_iact_force;
mod hydro_ghost;

#[derive(Default)]
pub struct Cell {
    domain: SimulationDomain2D,
    particles: Option<Vec<Particle>>,
    pub progeny: Option<[Box<Cell>; 4]>,
    pub del_tess: Option<DelaunayTriangulation2D>,
    pub vor_tess: Option<VoronoiGrid2D>,
    max_h: f64
}

impl Cell {
    pub fn from_dimensions(anchor: [f64; 2], sides: [f64; 2]) -> Self {
        Cell {
            domain: SimulationDomain2D::new(anchor, sides),
            particles: Some(vec![]),
            max_h: 0.,
            ..Cell::default() }
    }

    pub fn anchor(&self) -> [f64; 2] {
        self.domain.anchor()
    }

    pub fn sides(&self) -> [f64; 2] {
        self.domain.sides()
    }

    pub fn domain(&self) -> SimulationDomain2D {
        self.domain
    }

    pub fn add_particles(&mut self, x_values: &[f64], y_values: &[f64], h: f64) {
        for (&x, &y) in x_values.iter().zip(y_values.iter()) {
            self.add_particle(x, y, h);
        }
    }

    fn add_particle(&mut self, x: f64, y: f64, h: f64) {
        match self.particles.as_mut() {
            Some(particles) => {
                if h > self.max_h { self.max_h = h; }
                particles.push(Particle::new(x, y, h));
            },
            None => panic!("Trying to add a particle to cell which is not a leaf!")
        }
    }

    pub fn split(&mut self) {
        assert!(self.progeny.is_none() && self.particles.is_some(),
                "Trying to split a cell which is not a leaf!");

        // create progeny
        let anchor = self.anchor();
        let half_sides = [self.sides()[0] / 2., self.sides()[1] / 2.];
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
            progeny[pid as usize].add_particle(particle.x(), particle.y(), particle.h);
        }

        self.progeny = Some(progeny);
        self.particles = None;
    }

    pub fn delaunay_init(&mut self) {
        match self.particles.as_ref() {
            Some(particles) => {
                let n_parts = particles.len();
                self.del_tess = Some(DelaunayTriangulation2D::new(self.domain, n_parts, 2 * n_parts));
            }
            None => {
                for cell in self.progeny.as_mut().unwrap() {
                    cell.delaunay_init();
                }
            }
        }
    }
}
