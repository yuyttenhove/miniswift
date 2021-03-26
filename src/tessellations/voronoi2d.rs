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
        let mut grid = VoronoiGrid2D::with_capacity(triangulation.vertices.len(),
                                                    triangulation.triangles.len() - 3);
        // for each triangle of triangulation add the circumcenter to vertices (skip dummy triangles)
        for triangle in triangulation.triangles[3..].iter() {
            grid.vertices.push(triangulation.triangle_circumcenter(triangle));
        }

        // TODO: for each vertex of triangulation: find a triangle containing that vertex
        // TODO: loop around the vertex by jumping to neighbouring triangles, construct the
        // TODO: corresponding faces and build the area and centroid of the cell
        for (i, generator) in triangulation.vertices.iter().enumerate() {

        }

        // TODO divide area weighted sum of centroids by total area of cell -> centroid of cell
        grid
    }

    pub fn to_str(&self) -> String {
        let mut result = String::from("# Vertices #\n");
        for (i, v) in self.vertices.iter().enumerate() {
            result += &format!("{}\t({}, {})\n", i, v.x(), v.y());
        }

        result += "\n# cells #\n";
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