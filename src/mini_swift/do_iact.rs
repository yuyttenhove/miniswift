use crate::mini_swift::Cell;
use crate::random_points;
use crate::mini_swift::direction::Direction;

pub fn do_iact_test() {
    // setup + first density interactions
    let mut ci = Cell::from_dimensions([0., 0.], [1., 1.]);
    let (x_values, y_values) = random_points(16, &ci.domain(), true);
    ci.add_particles(&x_values, &y_values, 0.25);
    ci.split();
    ci.delaunay_init();
    ci.iact_density_self();

    let mut cj = Cell::from_dimensions([1., 0.], [1., 1.]);
    let (x_values, y_values) = random_points(4, &cj.domain(), true);
    cj.add_particles(&x_values, &y_values, 0.5);
    cj.delaunay_init();
    cj.iact_density_self();

    ci.iact_density_pair(&mut cj, Direction::Right);
    cj.del_tess.as_ref().unwrap().to_file("test.txt");
    ci.progeny.as_ref().unwrap()[3].del_tess.as_ref().unwrap().to_file("test2.txt");

    // ghost density interactions
    let max_iter = 100;
    let mut cur_iter = 0;
    let mut n_updated_i = ci.update_search_radii();
    let mut n_updated_j = cj.update_search_radii();
    while (n_updated_i > 0 || n_updated_j > 0) && cur_iter < max_iter {
        ci.iact_density_self_ghost();
        ci.iact_density_pair(&mut cj, Direction::Right);
        n_updated_i = ci.update_search_radii();
        n_updated_j = cj.update_search_radii();
        cur_iter += 1;
    }
}