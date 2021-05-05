use crate::mini_swift::Cell;
use crate::mini_swift::direction::{Direction, get_direction, direction_as_shift, invert_direction};
use rand_distr::num_traits::abs;

impl Cell{
    fn iact_density_self_base(&mut self) {
        // add this cells particles to this cells Delaunay tesselation
        assert!(self.particles.is_some(), "Cells particles is None!");
        assert!(self.del_tess.is_some(), "Delaunay tesselation not yet initialized!");
        let del_tess = self.del_tess.as_mut().unwrap();
        for particle in self.particles.as_ref().unwrap() {
            del_tess.insert_vertex(particle.x(), particle.y());
        }
        del_tess.finalize();
    }

    pub fn iact_density_self(&mut self) {
        match self.progeny.as_mut() {
            Some(progeny) => {
                // Do self interactions for subcells
                for cell in progeny.iter_mut() {
                    cell.iact_density_self()
                }
                // Do pair interactions between cells
                for i in 0..4 {
                    for j in i+1..4 {
                        let (ci, cj): (&mut Box<Cell>, &mut Box<Cell>);
                        unsafe{
                            // j > i, so this is in fact safe
                            ci = &mut *(progeny.get_unchecked_mut(i) as *mut _);
                            cj = &mut *(progeny.get_unchecked_mut(j) as *mut _);
                        }
                        ci.iact_density_pair(cj, get_direction(ci.anchor(), cj.anchor()));
                    }
                }
            }
            None => self.iact_density_self_base()
        }
    }

    fn iact_density_pair_base(&mut self, other: &mut Cell, direction: Direction) {
        // first side
        let mut del_tess = self.del_tess.as_mut().unwrap();
        let mut sides = self.domain.sides();
        let mut max_dist_2 = sides[0] * sides[0];
        let mut shift = direction_as_shift(direction);
        let mut anchor = self.domain.anchor();
        for particle in other.particles.as_mut().unwrap() {
            let rel_x = particle.x() - shift[0]*sides[0] - anchor[0];
            let rel_y = particle.y() - shift[1]*sides[1] - anchor[1];
            // TODO check with h
            if rel_x*rel_x*abs(shift[0]) + rel_y*rel_y*abs(shift[1]) < max_dist_2 {
                del_tess.insert_ghost_vertex(particle.x(), particle.y(), direction);
            }
        }
        // other side
        del_tess = other.del_tess.as_mut().unwrap();
        sides = other.domain.sides();
        max_dist_2 = sides[0] * sides[0];
        shift = [-shift[0], -shift[1]];
        anchor = other.domain.anchor();
        for particle in self.particles.as_mut().unwrap() {
            let rel_x = particle.x() - shift[0]*sides[0] - anchor[0];
            let rel_y = particle.y() - shift[1]*sides[1] - anchor[1];
            // TODO check with h
            if rel_x*rel_x*abs(shift[0]) + rel_y*rel_y*abs(shift[1]) < max_dist_2 {
                del_tess.insert_ghost_vertex(particle.x(), particle.y(), invert_direction(direction));
            }
        }
    }

    pub fn iact_density_pair(&mut self, other: &mut Cell, direction: Direction) {
        match self.progeny.as_mut() {
            Some(progeny) => {
                for child in progeny.iter_mut() {
                    // TODO check with max h if needed...
                    child.iact_density_pair(other, direction);
                }
            }
            None => {
                match other.progeny.as_mut() {
                    Some(ohter_progeny) => {
                        for other_child in ohter_progeny.iter_mut() {
                            // TODO check with max h if needed...
                            self.iact_density_pair(other_child, direction);
                        }
                    }
                    None => self.iact_density_pair_base(other, direction)
                }
            }
        }
    }

}