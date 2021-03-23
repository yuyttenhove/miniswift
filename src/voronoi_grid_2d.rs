use crate::delaunay_triangulation_2d::DelaunayTriangulation2D;

/// A simple point in 2D space
#[derive(Debug, Default)]
struct Vertex2D {
    x: f64,
    y: f64
}


/// A face (line) between two cells in a voronoi grid
#[derive(Debug, Default)]
struct VoronoiFace2D {
    area: f64,
    midpoint: Vertex2D,
    adjacent_cells: [i32; 3]
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
    pub fn from_delaunay_triangulation(triangulation: &DelaunayTriangulation2D) -> VoronoiGrid2D {
        VoronoiGrid2D::default()
    }
}