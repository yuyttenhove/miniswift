mod simulation_domain_2d;
mod geometry;
mod tessellations;
mod utils;

use tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};
use simulation_domain_2d::SimulationDomain2D;

fn main() {
    let domain = SimulationDomain2D::new([0., 0.], [1., 1.]);
    let mut d = DelaunayTriangulation2D::new(domain, 10, 10);
    println!("{:?}", d);
    for _ in 0..50 {
        d.insert_point(rand::random::<f64>(), rand::random::<f64>());
    }

    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt")
}
