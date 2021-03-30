use simulation_domain_2d::SimulationDomain2D;
use tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};
use rand_distr::Distribution;

mod simulation_domain_2d;
mod tessellations;
mod utils;

fn main() {
    let domain = SimulationDomain2D::new([0., 0.], [1., 1.]);
    let mut d = DelaunayTriangulation2D::new(domain, 10, 10);
    println!("{:?}", d);

    let uniform = true;
    if uniform {
        for _ in 0..200 {
            d.insert_point(rand::random::<f64>(), rand::random::<f64>());
        }
    } else {
        let normal = rand_distr::Normal::new(0.5, 0.2).unwrap();
        let mut rng = rand::thread_rng();
        for _ in 0..250 {
            // d.insert_point(rand::random::<f64>(), rand::random::<f64>());
            let x = normal.sample(&mut rng);
            let y = normal.sample(&mut rng);
            d.insert_point(x, y);
        }
    }

    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt")
}
