use super::delaunay2d::DelaunayTriangulation2D;
use super::{Vertex2D, Triangle2D};
use std::fs;
use std::iter::FromIterator;
use crate::simulation_domain_2d::SimulationDomain2D;


/// A face (line) between two cells in a voronoi grid
#[derive(Debug, Default)]
struct VoronoiFace2D {
    area: f64,
    midpoint: Vertex2D,
    adjacent_cells: [i32; 2]
}


/// A cell from a voronoi grid in 2D
#[derive(Debug)]
struct VoronoiCell2D {
    vertices: Vec<i32>,
    faces: Vec<i32>,
    centroid: Vertex2D,
    volume: f64
}

impl Default for VoronoiCell2D {
    fn default() -> VoronoiCell2D {
        VoronoiCell2D{
            vertices: Vec::new(),
            faces: Vec::new(),
            centroid: Vertex2D{x: 0., y: 0.0},
            volume: 0.
        }
    }
}

#[derive(Default, Debug)]
pub struct VoronoiGrid2D {
    vertices: Vec<Vertex2D>,
    faces: Vec<VoronoiFace2D>,
    cells: Vec<VoronoiCell2D>,
    is_periodic: bool,
    domain: SimulationDomain2D,
    n_cells: usize
}

impl VoronoiGrid2D {
    pub fn with_capacity(n_cells: usize, n_vertices: usize) -> VoronoiGrid2D {
        VoronoiGrid2D {
            vertices: Vec::with_capacity(n_vertices),
            faces: Vec::with_capacity(2 * n_vertices),
            cells: Vec::with_capacity(n_cells),
            ..VoronoiGrid2D::default()
        }
    }

    pub fn from_delaunay_triangulation(triangulation: &DelaunayTriangulation2D) -> VoronoiGrid2D {
        let mut grid = VoronoiGrid2D::with_capacity(triangulation.vertices.len() - 3,
                                                    triangulation.triangles.len() - 3);
        grid.n_cells = triangulation.n_vertices;
        grid.is_periodic = triangulation.is_periodic;
        // assert!(grid.is_periodic, "Non periodic grids are not yet supported!");
        grid.domain = triangulation.domain;
        // for each triangle of triangulation add the circumcenter to vertices (skip dummy triangles)
        for triangle in triangulation.triangles[3..].iter() {
            grid.vertices.push(triangle.circumcenter(triangulation));
        }

        // For each vertex of triangulation (generator): loop around the Delaunay triangles containing
        // that vertex. The circumcenters of those triangles are the voronoi vertices of the cell
        // generated by the current generator. Update the area and centroid of the current cell as
        // you go.
        for i in 3..triangulation.vertices.len() {
            if triangulation.is_connected_to_non_dummy_non_ghost_vertex(i){
                grid.add_cell_from_delaunay_generator(i, triangulation);
            }
        }
        grid
    }

    pub fn from_points(points_x: &Vec<f64>,
                       points_y: &Vec<f64>,
                       simulation_domain: SimulationDomain2D,
                       make_periodic: bool) -> VoronoiGrid2D {
        let delaunay = DelaunayTriangulation2D::from_points(
            points_x,
            points_y,
            simulation_domain,
            make_periodic
        );
        VoronoiGrid2D::from_delaunay_triangulation(&delaunay)
    }

    fn add_cell_from_delaunay_generator(&mut self, generator_idx: usize, triangulation: &DelaunayTriangulation2D) {
        let generator = &triangulation.vertices[generator_idx];
        let current_voronoi_cell_idx = generator_idx as i32 - 3;
        let mut idx_in_current_triangle = generator.index_in_triangle;
        let mut current_cell = VoronoiCell2D::default();
        let generator_as_vertex2d = Vertex2D{x: generator.x, y: generator.y };

        for current_triangle_idx_in_d in triangulation.get_triangle_idx_around_vertex(generator_idx) {
            let current_triangle = &triangulation.triangles[current_triangle_idx_in_d];
            assert_eq!(current_voronoi_cell_idx + 3, current_triangle.vertices[idx_in_current_triangle as usize]);
            let next_triangle_idx_in_current_triangle = ((idx_in_current_triangle + 1) % 3) as usize;
            let next_triangle_idx_in_d = current_triangle.neighbours[next_triangle_idx_in_current_triangle];

            let current_voronoi_vertex_idx = current_triangle_idx_in_d as i32 - 3;
            let next_voronoi_vertex_idx: i32 = next_triangle_idx_in_d - 3;
            current_cell.vertices.push(current_voronoi_vertex_idx);

            if generator_idx < triangulation.n_vertices + 3 {
                // create faces between cells
                let neighbouring_generator_idx_in_d = current_triangle.vertices[((idx_in_current_triangle + 2) % 3) as usize];
                let neighbouring_voronoi_cell_idx = neighbouring_generator_idx_in_d - 3;
                current_cell.faces.push(
                    self.get_or_create_face(
                        current_voronoi_vertex_idx,
                        next_voronoi_vertex_idx,
                        current_voronoi_cell_idx,
                        neighbouring_voronoi_cell_idx
                    )
                );
                // Update area and area weighted centroid sum of cell
                let current_wedge = Triangle2D::new(
                    generator_as_vertex2d,
                    self.vertices[current_voronoi_vertex_idx as usize],
                    self.vertices[next_voronoi_vertex_idx as usize]
                );
                let current_wedge_area = current_wedge.area();
                assert!(current_wedge_area >= 0.);
                current_cell.volume += current_wedge_area;
                current_cell.centroid += current_wedge_area * current_wedge.centroid();
            }
            // update idx_in_current_triangle
            let current_triangle_idx_in_next_triangle = current_triangle.index_in_neighbours[next_triangle_idx_in_current_triangle];
            idx_in_current_triangle = (current_triangle_idx_in_next_triangle + 1) % 3;
        }
        // Divide area weighted sum of centroids by total area of cell -> centroid of cell
        current_cell.centroid /= current_cell.volume;
        self.cells.push(current_cell);
    }

    fn get_or_create_face(&mut self, vertex_from_idx: i32, vertex_to_idx: i32, cell_in_idx: i32, cell_out_idx: i32) -> i32 {
        assert_ne!(cell_in_idx, cell_out_idx, "Trying to add face between a cell and itself!");
        assert_ne!(vertex_from_idx, vertex_to_idx, "Trying to add a face from a vertex to itself!");
        let face_idx: i32;
        if cell_out_idx < cell_in_idx && cell_out_idx > 2 {
            let cell_out = &self.cells[cell_out_idx as usize];
            let face_idx_in_cell_out = cell_out.vertices.iter().position(|&v_idx| v_idx == vertex_to_idx).unwrap();
            face_idx = cell_out.faces[face_idx_in_cell_out];
            assert_eq!(&self.faces[face_idx as usize].adjacent_cells[1], &cell_in_idx,
                       "Face has wrong adjacent cells!");
        } else {
            // create new face and return index
            face_idx = self.faces.len() as i32;
            self.faces.push(
                VoronoiFace2D{
                    area: (self.vertices[vertex_to_idx as usize] - self.vertices[vertex_from_idx as usize]).norm(),
                    midpoint: (self.vertices[vertex_to_idx as usize] + self.vertices[vertex_from_idx as usize]) / 2.,
                    adjacent_cells: [cell_in_idx, cell_out_idx]
                }
            );
        }
        face_idx
    }

    pub fn lloyd_relax(&self, move_threshold: f64, max_iter: usize) -> VoronoiGrid2D {
        let mut generators_x = Vec::from_iter(self.cells[..self.n_cells].iter().map(|c| c.centroid.x));
        let mut generators_y = Vec::from_iter(self.cells[..self.n_cells].iter().map(|c| c.centroid.y));

        let mut d = DelaunayTriangulation2D::from_points(
            &generators_x,
            &generators_y,
            self.domain,
            self.is_periodic
        );
        let mut v: VoronoiGrid2D = VoronoiGrid2D::from_delaunay_triangulation(&d);

        let mut displacement_threshold_satisfied = false;
        let mut previous_max_displacement = f64::INFINITY;
        let mut iter: usize = 0;
        while iter < max_iter && !displacement_threshold_satisfied {
            let mut max_displacement = 0.;
            for (i, voronoi_cell) in v.cells[..v.n_cells].iter().enumerate() {
                let centroid = voronoi_cell.centroid;
                let generator = Vertex2D{x: generators_x[i], y: generators_y[i]};
                let displacement = (generator - centroid).norm();
                if displacement > max_displacement {
                    max_displacement = displacement
                }
            }
            if max_displacement > previous_max_displacement { break; }
            previous_max_displacement = max_displacement;
            displacement_threshold_satisfied = max_displacement < move_threshold;
            iter += 1;
            generators_x = Vec::from_iter(v.cells[..v.n_cells].iter().map(|c| c.centroid.x));
            generators_y = Vec::from_iter(v.cells[..v.n_cells].iter().map(|c| c.centroid.y));
            d = DelaunayTriangulation2D::from_points(
                &generators_x,
                &generators_y,
                self.domain,
                self.is_periodic
            );
            v = VoronoiGrid2D::from_delaunay_triangulation(&d);
            println!("Relaxation iter: {}, maximum displacement was: {}", iter, max_displacement);
        }
        v
    }

    pub fn to_str(&self) -> String {
        let mut result = String::from("# Vertices #\n");
        for (i, v) in self.vertices.iter().enumerate() {
            result += &format!("{}\t({}, {})\n", i, v.x, v.y);
        }

        result += "\n# Cells #\n";
        for (i, cell) in self.cells.iter().enumerate() {
            result += &format!("{}\t(", i);
            for (i, vertex_idx) in cell.vertices.iter().enumerate(){
                result += &format!("{}", vertex_idx);
                if i < cell.vertices.len() - 1 {
                    result += ", "
                }
            }
            result += ")\n";
        }

        result += "\n# Centroids #\n";
        for (i, cell) in self.cells.iter().enumerate() {
            result += &format!("{}\t({}, {})\n", i, cell.centroid.x, cell.centroid.y);
        }
        result
    }

    pub fn to_file(&self, filename: &str) {
        fs::write(filename, self.to_str()).expect("Unable to write to file!");
    }
}