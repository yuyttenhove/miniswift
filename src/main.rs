use simulation_domain_2d::SimulationDomain2D;
use tessellations::{DelaunayTriangulation2D, VoronoiGrid2D};
use rand_distr::Distribution;
use crate::mini_swift::do_iact_test;
use crate::mini_swift::direction::Direction;
use rand::rngs::StdRng;
use rand::SeedableRng;

mod simulation_domain_2d;
mod tessellations;
mod utils;
mod mini_swift;


fn random_points(n: i32, domain: &SimulationDomain2D, uniform: bool, rng: &mut StdRng) -> (Vec<f64>, Vec<f64>) {
    let mut x_values = Vec::<f64>::new();
    let mut y_values = Vec::<f64>::new();
    if uniform {
        let uniform = rand::distributions::Uniform::from(0.0..1.0);
        for _ in 0..n {
            x_values.push(domain.sides()[0] * uniform.sample(rng) + domain.anchor()[0]);
            y_values.push(domain.sides()[1] * uniform.sample(rng) + domain.anchor()[1]);
        }
    } else {
        let normal = rand_distr::Normal::new(0.5, 0.15).unwrap();
        for _ in 0..n {
            x_values.push(domain.sides()[0] * normal.sample(rng) + domain.anchor()[0]);
            y_values.push(domain.sides()[1] * normal.sample(rng) + domain.anchor()[1]);
        }
    }
    (x_values, y_values)
}


fn main() {
    let side = 1.;
    let domain = SimulationDomain2D::new([0., 0.], [side, side]);


    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let (x_values, y_values) = random_points(100, &domain, true, &mut rng);

    let d = DelaunayTriangulation2D::from_points(&x_values, &y_values, domain, true);

    let g = VoronoiGrid2D::from_delaunay_triangulation(&d);
    // println!("{:?}", g);
    d.to_file("delaunay.txt");
    g.to_file("voronoi.txt");

    let g_relax = g.lloyd_relax(0.001, 10);
    g_relax.to_file("voronoi_relaxed.txt");

    do_iact_test();
}
