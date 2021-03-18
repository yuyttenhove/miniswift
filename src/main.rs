mod delaunay_triangulation_2d;
mod simulation_domain_2d;
mod geometry;

use delaunay_triangulation_2d::{DelaunayTriangulation2D};
use simulation_domain_2d::SimulationDomain2D;

fn main() {
    let domain = SimulationDomain2D::new([0., 0.], [1., 1.]);
    let mut d = DelaunayTriangulation2D::new(domain, 10, 10);
    println!("{:?}", d);
    d.insert_point(0.5, 0.5);
    d.insert_point(0.75, 0.95);
    println!("{:?}", d);
}
