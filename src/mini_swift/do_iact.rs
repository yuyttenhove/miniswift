use crate::mini_swift::Cell;
use crate::random_points;
use crate::mini_swift::direction::Direction;

use rand::SeedableRng;


fn init_cells_4_by_4() -> (Cell, Cell, Cell, Cell) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut ci = Cell::from_dimensions([0., 0.], [1., 1.]);
    let (x_values, y_values) = random_points(40, &ci.domain(), true, &mut rng);
    ci.add_particles(&x_values, &y_values, 0.1);
    ci.split();
    ci.delaunay_init();
    ci.iact_density_self();

    let mut cj = Cell::from_dimensions([1., 0.], [1., 1.]);
    let (x_values, y_values) = random_points(25, &cj.domain(), true, &mut rng);
    cj.add_particles(&x_values, &y_values, 0.1);
    cj.delaunay_init();
    cj.iact_density_self();

    let mut ck = Cell::from_dimensions([0., 1.], [1., 1.]);
    let (x_values, y_values) = random_points(40, &ck.domain(), true, &mut rng);
    ck.add_particles(&x_values, &y_values, 0.1);
    ck.split();
    ck.delaunay_init();
    ck.iact_density_self();

    let mut cl = Cell::from_dimensions([1., 1.], [1., 1.]);
    let (x_values, y_values) = random_points(25, &cl.domain(), true, &mut rng);
    cl.add_particles(&x_values, &y_values, 0.1);
    cl.delaunay_init();
    cl.iact_density_self();

    (ci, cj, ck, cl)
}

// cells are assumed to be in this orientation: ck cl
//                                              ci cj
fn do_pair_iact_periodic(ci: &mut Cell, cj: &mut Cell, ck: &mut Cell, cl: &mut Cell) {
    ci.iact_density_pair_shift(cl, Direction::RightDown, [0., -2.]);
    ci.iact_density_pair(cj, Direction::Right);
    ci.iact_density_pair(cl, Direction::RightUp);
    ci.iact_density_pair(ck, Direction::Up);

    cj.iact_density_pair_shift(ck, Direction::RightDown, [2., -2.]);
    cj.iact_density_pair_shift(ci, Direction::Right, [2., 0.]);
    cj.iact_density_pair_shift(ck, Direction::RightUp, [2., 0.]);
    cj.iact_density_pair(cl, Direction::Up);

    ck.iact_density_pair(cj, Direction::RightDown);
    ck.iact_density_pair(cl, Direction::Right);
    ck.iact_density_pair_shift(cj, Direction::RightUp, [2., 0.]);
    ck.iact_density_pair_shift(ci, Direction::Up, [0., 2.]);

    cl.iact_density_pair_shift(ci, Direction::RightDown, [2., 0.]);
    cl.iact_density_pair_shift(ck, Direction::Right, [2., 0.]);
    cl.iact_density_pair_shift(ci, Direction::RightUp, [2., 2.]);
    cl.iact_density_pair_shift(cj, Direction::Up, [0., 2.])
}

fn do_ghost(ci: &mut Cell, cj: &mut Cell, ck: &mut Cell, cl: &mut Cell) {
    let max_iter = 100;
    let mut cur_iter = 0;
    let mut n_updated_i = ci.update_search_radii();
    let mut n_updated_j = cj.update_search_radii();
    let mut n_updated_k = ck.update_search_radii();
    let mut n_updated_l = cl.update_search_radii();
    while (n_updated_i > 0 || n_updated_j > 0 || n_updated_k > 0 || n_updated_l > 0) && cur_iter < max_iter {
        ci.iact_density_self_ghost();
        cj.iact_density_self_ghost();
        ck.iact_density_self_ghost();
        cl.iact_density_self_ghost();

        do_pair_iact_periodic(ci, cj, ck, cl);

        n_updated_i = ci.update_search_radii();
        n_updated_j = cj.update_search_radii();
        n_updated_k = ck.update_search_radii();
        n_updated_l = cl.update_search_radii();
        cur_iter += 1;
    }
}

pub fn do_iact_test() {
    let (mut ci, mut cj, mut ck, mut cl) = init_cells_4_by_4();

    do_pair_iact_periodic(&mut ci, &mut cj, &mut ck, &mut cl);

    do_ghost(&mut ci, &mut cj, &mut ck, &mut cl);

    // print del_tess
    cj.del_tess.as_ref().unwrap().to_file("test.txt");
    // ci.del_tess.as_ref().unwrap().to_file("test2.txt");
    ci.progeny.as_ref().unwrap()[3].del_tess.as_ref().unwrap().to_file("test2.txt");
}