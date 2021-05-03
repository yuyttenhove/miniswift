use simulation_domain_2d::SimulationDomain2D;
use tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};
use rand_distr::Distribution;
use rand::SeedableRng;

mod simulation_domain_2d;
mod tessellations;
mod utils;
mod mini_swift;


fn main() {
    let side = 1.;
    let domain = SimulationDomain2D::new([0., 0.], [side, side]);

    let mut x_values = Vec::<f64>::new();
    let mut y_values = Vec::<f64>::new();

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let uniform = true;
    if uniform {
        let uniform = rand::distributions::Uniform::from(0.0..side);
        for _ in 0..100 {
            x_values.push(uniform.sample(&mut rng));
            y_values.push(uniform.sample(&mut rng));
        }
    } else {
        let normal = rand_distr::Normal::new(0.5, 0.15).unwrap();
        for _ in 0..200 {
            // d.insert_point(rand::random::<f64>(), rand::random::<f64>());
            x_values.push(normal.sample(&mut rng));
            y_values.push(normal.sample(&mut rng));
        }
    }

    let d = DelaunayTriangulation2D::from_points(&x_values, &y_values, domain, true);

    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    // println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt");

    let g_relax = g.lloyd_relax(0.000001, 10);
    g_relax.to_file("voronoi_relaxed.txt")
}
