use simulation_domain_2d::SimulationDomain2D;
use tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};
use rand_distr::Distribution;
use rand::SeedableRng;

mod simulation_domain_2d;
mod tessellations;
mod utils;

fn main() {
    let domain = SimulationDomain2D::new([0., 0.], [1., 1.]);
    let mut d = DelaunayTriangulation2D::new(&domain, 10, 10);
    println!("{:?}", d);

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let uniform = true;
    if uniform {
        let uniform = rand::distributions::Uniform::from(0.0..1.0);
        for _ in 0..2 {
            d.insert_point(uniform.sample(&mut rng), uniform.sample(&mut rng));
        }
    } else {
        let normal = rand_distr::Normal::new(0.5, 0.1).unwrap();
        for _ in 0..5 {
            // d.insert_point(rand::random::<f64>(), rand::random::<f64>());
            let x = normal.sample(&mut rng);
            let y = normal.sample(&mut rng);
            d.insert_point(x, y);
        }
    }

    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt");

    // let g_relax = g.lloyd_relax(0.00001, 100);
    // g_relax.to_file("voronoi_relaxed.txt")
}
