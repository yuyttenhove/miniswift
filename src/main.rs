use simulation_domain_2d::SimulationDomain2D;
use tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};
use rand_distr::Distribution;
use rand::SeedableRng;

mod simulation_domain_2d;
mod tessellations;
mod utils;

fn main() {
    let domain = SimulationDomain2D::new([0., 0.], [1., 1.]);

    let mut xvals = Vec::<f64>::new();
    let mut yvals = Vec::<f64>::new();

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let uniform = true;
    if uniform {
        let uniform = rand::distributions::Uniform::from(0.0..1.0);
        for _ in 0..100 {
            xvals.push(uniform.sample(&mut rng));
            yvals.push(uniform.sample(&mut rng));
        }
    } else {
        let normal = rand_distr::Normal::new(0.5, 0.1).unwrap();
        for _ in 0..5 {
            // d.insert_point(rand::random::<f64>(), rand::random::<f64>());
            xvals.push(normal.sample(&mut rng));
            yvals.push(normal.sample(&mut rng));
        }
    }

    let d = DelaunayTriangulation2D::from_points(&xvals, &yvals, domain, true);

    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt");

    // let g_relax = g.lloyd_relax(0.00001, 100);
    // g_relax.to_file("voronoi_relaxed.txt")
}
