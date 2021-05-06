use crate::mini_swift::Cell;
use crate::tessellations::VoronoiGrid2D;
use std::panic::panic_any;

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
                        particle.h *= 1.25;
                        n_updated += 1;
                        if particle.h > self.max_h {
                            self.max_h = particle.h;
                        }
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

    pub fn end_density(&mut self) {
        match self.progeny.as_mut() {
            Some(progeny) => {
                for cell in progeny {
                    cell.end_density();
                }
            }
            None => {
                match self.del_tess.as_ref(){
                    Some(del_tess) => {
                        self.vor_tess = Some(VoronoiGrid2D::from_delaunay_triangulation(del_tess));
                    }
                    None => panic!("Trying to construct voronoi grid for cell without delaunay triangulation!")
                }
            }
        }
    }
}