mod delaunay_triangulation_2d;
mod simulation_domain_2d;
mod geometry;
mod voronoi_grid_2d;

use delaunay_triangulation_2d::{DelaunayTriangulation2D};
use simulation_domain_2d::SimulationDomain2D;
use voronoi_grid_2d::VoronoiGrid2D;

fn main() {
    let domain = SimulationDomain2D::new([0., 0.], [1., 1.]);
    let mut d = DelaunayTriangulation2D::new(domain, 10, 10);
    println!("{:?}", d);

    for _ in 0..100 {
        d.insert_point(rand::random::<f64>(), rand::random::<f64>());
    }
    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt")
}
