use crate::mini_swift::Cell;
use crate::mini_swift::direction::{Direction, get_direction, direction_as_vector, invert_direction, direction_to_sort_list_id};
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

    fn iact_density_self_sub_pairs(&mut self) {
        let progeny = self.progeny.as_mut().unwrap();
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

    pub fn iact_density_self(&mut self) {
        match self.progeny.as_mut() {
            Some(progeny) => {
                // Do self interactions for subcells
                for cell in progeny.iter_mut() {
                    cell.iact_density_self()
                }
                // Do pair interactions between cells
                self.iact_density_self_sub_pairs();
            }
            None => self.iact_density_self_base()
        }
    }

    pub fn iact_density_self_ghost(&mut self) {
        match self.progeny.as_mut() {
            Some(progeny) => {
                // Do only pair interactions between cells
                self.iact_density_self_sub_pairs();
            }
            None => ()
        }
    }

    fn iact_density_pair_base(&mut self, other: &mut Cell, direction: Direction, shift: [f64; 2]) {
        let del_tess = self.del_tess.as_mut().unwrap();
        let other_del_tess = other.del_tess.as_mut().unwrap();
        let sid = direction_to_sort_list_id(direction);
        let inv_direction = invert_direction(direction);
        let inv_sid = direction_to_sort_list_id(inv_direction);

        for particle in self.particles.as_mut().unwrap() {
            for other_particle in other.particles.as_mut().unwrap() {
                if other_particle.x() == 1.0502736853580725 {
                    let a = 0;
                }
                let x = particle.x();
                let y = particle.y();
                let other_x = (other_particle.x() + shift[0]);
                let other_y = (other_particle.y() + shift[1]);
                let delta_x = x - other_x;
                let delta_y = y - other_y;
                let dist_2 = delta_x * delta_x + delta_y * delta_y;
                // first direction
                // TODO symmetrize?
                if dist_2 < particle.h * particle.h {
                    if other_particle.added_to_del_tess & 1 << inv_sid == 0 {
                        del_tess.insert_ghost_vertex(other_x, other_y, direction);
                        other_particle.added_to_del_tess |= 1 << inv_sid;
                    }
                }
                // the other direction
                if dist_2 < other_particle.h * other_particle.h {
                    if particle.added_to_del_tess & 1 << sid == 0 {
                        other_del_tess.insert_ghost_vertex(x - shift[0], y - shift[1], invert_direction(direction));
                        particle.added_to_del_tess |= 1 << sid;
                    }
                }
            }
        }
    }

    pub fn iact_density_pair_shift(&mut self, other: &mut Cell, direction: Direction, shift: [f64; 2]) {
        if !self.can_interact(other, direction, shift) { return; }

        match self.progeny.as_mut() {
            Some(progeny) => {
                for child in progeny.iter_mut() {
                    child.iact_density_pair_shift(other, direction, shift);
                }
            }
            None => {
                match other.progeny.as_mut() {
                    Some(ohter_progeny) => {
                        for other_child in ohter_progeny.iter_mut() {
                            self.iact_density_pair_shift(other_child, direction, shift);
                        }
                    }
                    None => self.iact_density_pair_base(other, direction, shift)
                }
            }
        }
    }

    pub fn iact_density_pair(&mut self, other: &mut Cell, direction: Direction) {
        self.iact_density_pair_shift(other, direction, [0., 0.]);
    }

    fn can_interact(&self, other: &Cell, direction: Direction, shift: [f64; 2]) -> bool {
        // Calculate minimal possible distance between a point of self and other
        let direction_vec = direction_as_vector(direction);
        let min_delta_x = direction_vec[0] * (self.anchor()[0] + self.sides()[0]* direction_vec[0] - (other.anchor()[0] + shift[0]));
        let min_delta_y = direction_vec[1] * (self.anchor()[1] + self.sides()[1]* direction_vec[1] - (other.anchor()[1] + shift[1]));
        let min_dist = min_delta_x * min_delta_x + min_delta_y * min_delta_y;
        // Can interact?
        min_dist < self.max_h * self.max_h || min_dist < other.max_h * other.max_h
    }

}