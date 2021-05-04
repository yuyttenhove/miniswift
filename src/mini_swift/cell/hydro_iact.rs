use crate::mini_swift::Cell;

impl Cell{
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
                        ci.iact_density_pair(cj);
                    }
                }
            }
            None => {
                // add this cells particles to this cells Delaunay tesselation
                assert!(self.particles.is_some(), "Cells particles is None!");
                assert!(self.del_tess.is_some(), "Delaunay tesselation not yet initialized!");
                let del_tess = self.del_tess.as_mut().unwrap();
                for particle in self.particles.as_ref().unwrap() {
                    del_tess.insert_point(particle.x(), particle.y());
                }
            }
        }
    }

    pub fn iact_density_pair(&mut self, other: &mut Cell) {}

}