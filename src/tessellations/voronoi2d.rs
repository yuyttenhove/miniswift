use super::delaunay2d::DelaunayTriangulation2D;
use super::{Vertex2D, Triangle2D};
use std::fs;


/// A face (line) between two cells in a voronoi grid
#[derive(Debug, Default)]
struct VoronoiFace2D {
    area: f64,
    midpoint: Vertex2D,
    adjacent_cells: [i32; 2]
}


/// A cell from a voronoi grid in 2D
#[derive(Debug, Default)]
struct VoronoiCell2D {
    vertices: Vec<i32>,
    faces: Vec<i32>,
    centroid: Vertex2D,
    volume: f64
}

#[derive(Default, Debug)]
pub struct VoronoiGrid2D {
    vertices: Vec<Vertex2D>,
    faces: Vec<VoronoiFace2D>,
    cells: Vec<VoronoiCell2D>
}

impl VoronoiGrid2D {
    pub fn with_capacity(n_generators: usize, n_triangles: usize) -> VoronoiGrid2D {
        VoronoiGrid2D {
            vertices: Vec::with_capacity(n_triangles),
            faces: Vec::with_capacity(2 * n_triangles),
            cells: Vec::with_capacity(n_generators)
        }
    }

    pub fn from_delaunay_triangulation(triangulation: &DelaunayTriangulation2D) -> VoronoiGrid2D {
        let mut grid = VoronoiGrid2D::with_capacity(triangulation.vertices.len() - 3,
                                                    triangulation.triangles.len() - 3);
        // for each triangle of triangulation add the circumcenter to vertices (skip dummy triangles)
        for triangle in triangulation.triangles[3..].iter() {
            grid.vertices.push(triangle.circumcenter(triangulation));
        }

        // For each vertex of triangulation (generator): find a triangle containing that vertex
        // Loop around the vertex by jumping to neighbouring triangles, construct the cell
        // corresponding to that generator
        for (i, generator) in triangulation.vertices[3..].iter().enumerate() {
            let current_voronoi_cell_idx = i as i32;
            let start_triangle_idx_in_d = generator.triangle;
            let mut current_triangle_idx_in_d = start_triangle_idx_in_d;
            let mut idx_in_current_triangle = generator.index_in_triangle;
            let mut current_cell = VoronoiCell2D{
                centroid: Vertex2D{x: 0., y: 0.},
                ..VoronoiCell2D::default()
            };
            let generator_as_vertex2d = Vertex2D{x: generator.x, y: generator.y};

            let mut previous_triangle_idx_in_d = -1;
            let mut n_neighbours_processed = 0;

            while previous_triangle_idx_in_d != start_triangle_idx_in_d
                    || n_neighbours_processed < 2 {
                let current_triangle = &triangulation.triangles[current_triangle_idx_in_d as usize];
                assert_eq!(current_voronoi_cell_idx + 3, current_triangle.vertices[idx_in_current_triangle as usize]);

                let next_triangle_idx_in_current_triangle = ((idx_in_current_triangle + 1) % 3) as usize;
                let next_triangle_idx_in_d = current_triangle.neighbours[next_triangle_idx_in_current_triangle];
                let current_triangle_idx_in_next_triangle = current_triangle.index_in_neighbours[next_triangle_idx_in_current_triangle];

                let current_voronoi_vertex_idx = current_triangle_idx_in_d - 3;
                let next_voronoi_vertex_idx: i32 = next_triangle_idx_in_d - 3;
                current_cell.vertices.push(current_voronoi_vertex_idx);

                // create faces between cells
                let neighbouring_generator_idx_in_d = current_triangle.vertices[((idx_in_current_triangle + 2) % 3) as usize];
                let neighbouring_voronoi_cell_idx = neighbouring_generator_idx_in_d - 3;
                current_cell.faces.push(
                    grid.get_or_create_face(
                        current_voronoi_vertex_idx,
                        next_voronoi_vertex_idx,
                        current_voronoi_cell_idx,
                        neighbouring_voronoi_cell_idx
                    )
                );

                // Update area and area weighted centroid sum of cell
                let current_wedge = Triangle2D::new(
                    generator_as_vertex2d,
                    grid.vertices[current_voronoi_vertex_idx as usize],
                    grid.vertices[next_voronoi_vertex_idx as usize]
                );
                let current_wedge_area = current_wedge.area();
                assert!(current_wedge_area >= 0.);
                current_cell.volume += current_wedge_area;
                current_cell.centroid += current_wedge_area * current_wedge.centroid();

                previous_triangle_idx_in_d = current_triangle_idx_in_d;
                current_triangle_idx_in_d = next_triangle_idx_in_d;
                idx_in_current_triangle = (current_triangle_idx_in_next_triangle + 1) % 3;
                n_neighbours_processed += 1;
            }
            // Divide area weighted sum of centroids by total area of cell -> centroid of cell
            current_cell.centroid /= current_cell.volume;
            grid.cells.push(current_cell);
        }
        grid
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