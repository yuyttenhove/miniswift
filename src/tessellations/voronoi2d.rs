use crate::tessellations::delaunay2d::DelaunayTriangulation2D;
use crate::geometry::{Vertex2D};
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
            grid.vertices.push(triangulation.triangle_circumcenter(triangle));
        }

        // TODO: for each vertex of triangulation: find a triangle containing that vertex
        // TODO: loop around the vertex by jumping to neighbouring triangles, construct the
        // TODO: corresponding faces and build the area and centroid of the cell
        for (i, generator) in triangulation.vertices[3..].iter().enumerate() {
            let start_triangle_idx_in_d = generator.triangle;
            let mut current_triangle_idx_in_d = start_triangle_idx_in_d;
            let mut idx_in_current_triangle = generator.index_in_triangle;
            let mut current_cell = VoronoiCell2D::default();

            let mut previous_triangle_idx_in_d = -1;
            let mut n_neighbours_processed = 0;

            while previous_triangle_idx_in_d != start_triangle_idx_in_d
                    || n_neighbours_processed < 2 {
                let current_triangle = &triangulation.triangles[current_triangle_idx_in_d as usize];
                assert_eq!(i as i32 + 3, current_triangle.vertices[idx_in_current_triangle as usize]);

                let next_triangle_idx_in_current_triangle = ((idx_in_current_triangle + 1) % 3) as usize;
                let next_triangle_idx_in_d = current_triangle.neighbours[next_triangle_idx_in_current_triangle];
                let current_triangle_idx_in_next_triangle = current_triangle.index_in_neighbours[next_triangle_idx_in_current_triangle];
                current_cell.vertices.push(current_triangle_idx_in_d - 3);

                previous_triangle_idx_in_d = current_triangle_idx_in_d;
                current_triangle_idx_in_d = next_triangle_idx_in_d;
                idx_in_current_triangle = (current_triangle_idx_in_next_triangle + 1) % 3;
                n_neighbours_processed += 1;
            }

            grid.cells.push(current_cell);
        }

        // TODO divide area weighted sum of centroids by total area of cell -> centroid of cell
        grid
    }

    pub fn to_str(&self) -> String {
        let mut result = String::from("# Vertices #\n");
        for (i, v) in self.vertices.iter().enumerate() {
            result += &format!("{}\t({}, {})\n", i, v.x(), v.y());
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
        result
    }

    pub fn to_file(&self, filename: &str) {
        fs::write(filename, self.to_str()).expect("Unable to write to file!");
    }
}