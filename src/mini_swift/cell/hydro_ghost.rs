use crate::mini_swift::Cell;

impl Cell {
    pub fn update_search_radii(&mut self) -> u32 {
        let mut n_updated = 0;
        match self.particles.as_mut(){
            Some(particles) => {
                let del_tess = self.del_tess.as_mut().unwrap();
                del_tess.update_vertex_search_radii(0., del_tess.n_vertices);
                for (i, particle) in particles.iter_mut().enumerate() {
                    let search_radius = del_tess.vertices[i + 3].search_radius;
                    if del_tess.vertices[i + 3].search_radius > particle.h {
                        particle.h *= 1.5;
                        n_updated += 1;
                    }
                }
            }
            None => {
                for cell in self.progeny.as_mut().unwrap() {
                    n_updated += cell.update_search_radii();
                }
            }
        }
        n_updated
    }
}