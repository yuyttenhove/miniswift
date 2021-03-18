use crate::simulation_domain_2d::SimulationDomain2D;
use crate::geometry::orient_2d;
use rand;


fn random_choose<T>(option1: T, option2: T) -> T {
    if rand::random() {option1} else {option2}
}


#[derive(Debug)]
struct DelaunayVertex2D {
    x: f64,
    y: f64,
    triangle: i64,
    triangle_index: i8
}

impl Default for DelaunayVertex2D {
    fn default() -> DelaunayVertex2D {
        DelaunayVertex2D { x: f64::NAN, y: f64::NAN, triangle: -1, triangle_index: -1 }
    }
}

impl DelaunayVertex2D {
    fn update_triangle(&mut self, triangle: i64, triangle_index: i8) {
        self.triangle = triangle;
        self.triangle_index = triangle_index;
    }
}


#[derive(Debug)]
struct DelaunayTriangle2D {
    vertices: [i64; 3],
    neighbours: [i64; 3],
}

impl Default for DelaunayTriangle2D {
    fn default() -> DelaunayTriangle2D {
        DelaunayTriangle2D { vertices: [-1, -1, -1], neighbours: [-1, -1, -1] }
    }
}

impl DelaunayTriangle2D {
    fn update_neighbours(&mut self, n0: i64, n1: i64, n2: i64) {
        self.neighbours = [n0, n1, n2];
    }

    fn update_neighbour(&mut self, n: i64, i: usize) {
        self.neighbours[i] = n;
    }
}


#[derive(Debug, Default)]
pub struct DelaunayTriangulation2D {
    vertices: Vec<DelaunayVertex2D>,
    triangles: Vec<DelaunayTriangle2D>,
    anchor: [f64; 2],
    side: f64,
    inverse_side: f64,
    current_triangle: i64
}

impl DelaunayTriangulation2D {
    pub fn new(domain: SimulationDomain2D, vertex_size: usize, triangle_size: usize) -> DelaunayTriangulation2D {
        let mut triangulation = DelaunayTriangulation2D{
            vertices: Vec::with_capacity(vertex_size),
            triangles: Vec::with_capacity(triangle_size),
            ..DelaunayTriangulation2D::default()
        };

        /* Setup the domain and side of the triangulation box large enough so that any
        ghost particles certainly fall into the domain.
         */
        triangulation.anchor = [domain.anchor()[0] - domain.sides()[0],
                                domain.anchor()[1] - domain.sides()[1]];
        triangulation.side = 6. * f64::max(domain.sides()[0], domain.sides()[1]);
        triangulation.inverse_side = 1. / triangulation.side;

        // Create vertices for the first triangle that encapsulates the entire domain
        let v0 = triangulation.new_vertex(triangulation.anchor[0], triangulation.anchor[1]);
        let v1 = triangulation.new_vertex(triangulation.anchor[0] + triangulation.side,
                                          triangulation.anchor[1]);
        let v2 = triangulation.new_vertex(triangulation.anchor[0],
                                          triangulation.anchor[1] + triangulation.side);

        // Create first large triangle with 3 dummy triangles as neighbours
        let dummy0 = triangulation.new_triangle(v1, v2, -1);
        let dummy1 = triangulation.new_triangle(v2, v0, -1);
        let dummy2 = triangulation.new_triangle(v0, v1, -1);
        let first_triangle = triangulation.new_triangle(v0, v1, v2);
        triangulation.triangles[first_triangle as usize].update_neighbours(dummy0, dummy1, dummy2);
        triangulation.triangles[dummy0 as usize].update_neighbour(first_triangle, 0);
        triangulation.triangles[dummy1 as usize].update_neighbour(first_triangle, 1);
        triangulation.triangles[dummy2 as usize].update_neighbour(first_triangle, 2);

        triangulation.consistency_check();

        triangulation.current_triangle = first_triangle;
        triangulation
    }

    pub fn insert_point(&mut self, x: f64, y: f64) {
        let new_vertex_idx = self.new_vertex(x, y);
        // Find triangle in which (x, y) is positioned
        let triangle_idx = self.find_triangle_containing_vertex(&self.vertices[new_vertex_idx as usize]);
        let triangle = &self.triangles[triangle_idx as usize];

        // Create 3 new triangles
        let (v0, v1, v2) = (triangle.vertices[0], triangle.vertices[1], triangle.vertices[2]);
        let (n0, n1, n2) = (triangle.neighbours[0], triangle.neighbours[1], triangle.neighbours[2]);

        let triangle0 = self.new_triangle_at(v1, v2, new_vertex_idx, triangle_idx);
        let triangle1 = self.new_triangle(v2, v0, new_vertex_idx);
        let triangle2 = self.new_triangle(v0, v1, new_vertex_idx);

        // Update neighbours
        self.triangles[triangle0 as usize].update_neighbours(n0, triangle2, triangle1);
        self.triangles[n0 as usize].update_neighbour(triangle0, 0);
        self.triangles[triangle1 as usize].update_neighbours(triangle2, n1, triangle0);
        self.triangles[n1 as usize].update_neighbour(triangle1, 1);
        self.triangles[triangle2 as usize].update_neighbours(triangle1, triangle0, n2);
        self.triangles[n2 as usize].update_neighbour(triangle2, 2);


        // TODO: check new triangles for delaunay criterion and add to fixing queue if necessary

        // TODO: fix triangles until queue is empty

        self.consistency_check();
    }

    fn new_vertex(&mut self, x: f64, y: f64) -> i64 {
        // TODO possibly manage the size of self.vertices more intelligently.
        let vertex = self.vertices.len() as i64;
        self.vertices.push(DelaunayVertex2D{x, y, ..DelaunayVertex2D::default()});

        vertex
    }

    fn new_triangle(&mut self, v0: i64, v1: i64, v2: i64) -> i64 {
        self.new_triangle_at(v0, v1, v2, -1)
    }

    fn new_triangle_at(&mut self, v0: i64, v1: i64, v2: i64, mut at: i64) -> i64 {
        // TODO possibly manage the size of self.triangles more intelligently.
        if at < 0 {
            at = self.triangles.len() as i64;
            self.triangles.push(DelaunayTriangle2D{
                vertices: [v0, v1, v2],
                ..DelaunayTriangle2D::default()
            });
        } else {
            self.triangles[at as usize] = DelaunayTriangle2D{
                vertices: [v0, v1, v2],
                ..DelaunayTriangle2D::default()
            };
        }

        if v0 >= 0 {
            self.vertices[v0 as usize].update_triangle(at, 0);
        }
        if v1 >= 0 {
            self.vertices[v1 as usize].update_triangle(at, 1);
        }
        if v2 >= 0 {
            self.vertices[v2 as usize].update_triangle(at, 2);
        }

        at
    }

    fn find_triangle_containing_vertex(&self, vertex: &DelaunayVertex2D) -> i64 {
        let mut v0: &DelaunayVertex2D;
        let mut v1: &DelaunayVertex2D;
        let mut v2: &DelaunayVertex2D;
        let mut current_triangle: &DelaunayTriangle2D;
        let mut current_triangle_idx = self.current_triangle;
        let mut found = false;
        let mut test0: f64;
        let mut test1: f64;
        let mut test2: f64;

        while !found {
            current_triangle = &self.triangles[current_triangle_idx as usize];
            v0 = &self.vertices[current_triangle.vertices[0] as usize];
            v1 = &self.vertices[current_triangle.vertices[1] as usize];
            v2 = &self.vertices[current_triangle.vertices[2] as usize];

            test2 = orient_2d(v0.x, v0.y, v1.x, v1.y, vertex.x, vertex.y);
            test0 = orient_2d(v1.x, v1.y, v2.x, v2.y, vertex.x, vertex.y);
            test1 = orient_2d(v2.x, v2.y, v0.x, v0.y, vertex.x, vertex.y);

            if (test0 > 0.) && (test1 > 0.) && (test2 > 0.) {
                found = true;
            } else if (test0 <= 0.) && (test1 <= 0.) {
                current_triangle_idx = random_choose(current_triangle.neighbours[0],
                                                     current_triangle.neighbours[1]);
            } else if (test1 <= 0.) && (test2 <= 0.) {
                current_triangle_idx = random_choose(current_triangle.neighbours[1],
                                                     current_triangle.neighbours[2]);
            } else if (test0 <= 0.) && (test2 <= 0.) {
                current_triangle_idx = random_choose(current_triangle.neighbours[0],
                                                     current_triangle.neighbours[2]);
            } else if test0 <= 0. {
                current_triangle_idx = current_triangle.neighbours[0];
            } else if test1 <= 0.{
                current_triangle_idx = current_triangle.neighbours[1];
            } else if test2 <= 0.{
                current_triangle_idx = current_triangle.neighbours[2];
            } else {
                println!("Error! This scenario should not be possible?")
            }

            if current_triangle_idx == -1 {
                println!("Error! Ended up with dummy triangle")
            }
        }
        current_triangle_idx
    }

    fn consistency_check(&self) {
        // for each triangle check neighbour symmetry and check delaunay criterion
        // for each vertex check triangle information
        for (i, triangle) in self.triangles[3..].iter().enumerate() {
            let triangle_idx = (i + 3) as i64;
            for (j, &ngbr) in triangle.neighbours.iter().enumerate() {
                assert_eq!(triangle_idx,
                           self.triangles[ngbr as usize].neighbours[j],
                           "Testing neighbour symmetry");
            }
            // TODO check overlapping vertices?
            // TODO check circumcircle
        }

        for (i, vertex) in self.vertices.iter().enumerate() {
            assert_eq!(i as i64,
                       self.triangles[vertex.triangle as usize].vertices[vertex.triangle_index as usize],
                       "Testing vertex-triangle correspondence");
        }
    }
}