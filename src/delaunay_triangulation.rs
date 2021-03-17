
struct DelaunayVertex {
    x: f64,
    y: f64,
}


struct DelaunayTriangle {
    vertices: Vec<usize>,
    neighbours: Vec<usize>,
}


pub struct Delaunay{
    vertices: Vec<DelaunayVertex>,
    triangles: Vec<DelaunayTriangle>,
}

impl Delaunay {
    fn initialize(&mut self, vertices: Vec<DelaunayVertex>){
        self.vertices = vertices;
    }
}