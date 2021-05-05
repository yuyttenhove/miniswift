use crate::mini_swift::Cell;
use crate::random_points;
use crate::mini_swift::direction::Direction;

pub fn do_iact_test() {
    let mut ci = Cell::from_dimensions([0., 0.], [1., 1.]);
    let (x_values, y_values) = random_points(100, &ci.domain(), true);
    ci.add_particles(&x_values, &y_values, 0.1);
    ci.split();
    ci.delaunay_init();
    ci.iact_density_self();

    let mut cj = Cell::from_dimensions([1., 0.], [1., 1.]);
    let (x_values, y_values) = random_points(25, &cj.domain(), true);
    cj.add_particles(&x_values, &y_values, 0.25);
    cj.delaunay_init();
    cj.iact_density_self();

    ci.iact_density_pair(&mut cj, Direction::Right);
    cj.del_tess.as_ref().unwrap().to_file("test.txt");
    ci.progeny.as_ref().unwrap()[3].del_tess.as_ref().unwrap().to_file("test2.txt");
}