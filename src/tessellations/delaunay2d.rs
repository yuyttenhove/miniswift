use crate::simulation_domain_2d::SimulationDomain2D;
use crate::geometry::{orient_2d, in_circle_2d, circumcenter_2d, Vertex2D};
use crate::utils::random_choose;
use std::collections::VecDeque;
use std::fs;


#[derive(Debug)]
pub struct DelaunayVertex2D {
    x: f64,
    y: f64,
    triangle: i32,
    index_in_triangle: i8
}

impl Default for DelaunayVertex2D {
    fn default() -> DelaunayVertex2D {
        DelaunayVertex2D { x: f64::NAN, y: f64::NAN, triangle: -1, index_in_triangle: -1 }
    }
}

impl DelaunayVertex2D {
    fn update_triangle(&mut self, triangle: i32, triangle_index: i8) {
        self.triangle = triangle;
        self.index_in_triangle = triangle_index;
    }
}


#[derive(Debug)]
pub struct DelaunayTriangle2D {
    vertices: [i32; 3],
    neighbours: [i32; 3],
    index_in_neighbours: [i8; 3]
}

impl Default for DelaunayTriangle2D {
    fn default() -> DelaunayTriangle2D {
        DelaunayTriangle2D {
            vertices: [-1, -1, -1],
            neighbours: [-1, -1, -1],
            index_in_neighbours: [-1, -1, -1]
        }
    }
}

impl DelaunayTriangle2D {
    fn update_neighbours(&mut self, n0: i32, n1: i32, n2: i32, idx_in_n0: i8, idx_in_n1: i8, idx_in_n2: i8) {
        self.neighbours = [n0, n1, n2];
        self.index_in_neighbours = [idx_in_n0, idx_in_n1, idx_in_n2];
    }

    fn update_neighbour(&mut self, n: i32, idx_in_n: i8, i: i8) {
        self.neighbours[i as usize] = n;
        self.index_in_neighbours[i as usize] = idx_in_n;
    }
}


#[derive(Debug, Default)]
pub struct DelaunayTriangulation2D {
    pub(super) vertices: Vec<DelaunayVertex2D>,
    pub(super) triangles: Vec<DelaunayTriangle2D>,
    anchor: [f64; 2],
    side: f64,
    inverse_side: f64,
    current_triangle_idx: i32,
    current_vertex_idx: i32,
    triangles_to_check: VecDeque<i32>
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
        triangulation.triangles[first_triangle as usize].update_neighbours(dummy0, dummy1, dummy2,
                                                                           0, 1, 2);
        triangulation.triangles[dummy0 as usize].update_neighbour(first_triangle, 0, 0);
        triangulation.triangles[dummy1 as usize].update_neighbour(first_triangle, 1, 1);
        triangulation.triangles[dummy2 as usize].update_neighbour(first_triangle, 2, 2);

        triangulation.consistency_check();

        triangulation.current_triangle_idx = first_triangle;
        triangulation
    }

    pub fn insert_point(&mut self, x: f64, y: f64) {
        // add vertex
        self.new_vertex(x, y);

        // Find triangle in which (x, y) is positioned
        self.current_triangle_idx = self.find_triangle_containing_current_vertex();
        let triangle = &self.triangles[self.current_triangle_idx as usize];

        // Create 3 new triangles
        let (v0, v1, v2) = (triangle.vertices[0], triangle.vertices[1], triangle.vertices[2]);
        let (n0, n1, n2) = (triangle.neighbours[0], triangle.neighbours[1], triangle.neighbours[2]);
        let (idx_in_n0, idx_in_n1, idx_in_n2) = (
            triangle.index_in_neighbours[0],
            triangle.index_in_neighbours[1],
            triangle.index_in_neighbours[2]
        );

        let triangle0 = self.new_triangle_at(v1, v2, self.current_vertex_idx, self.current_triangle_idx);
        let triangle1 = self.new_triangle(v2, v0, self.current_vertex_idx);
        let triangle2 = self.new_triangle(v0, v1, self.current_vertex_idx);

        // Update neighbours
        self.triangles[triangle0 as usize].update_neighbours(triangle1, triangle2, n0,
                                                             1, 0, idx_in_n0);
        self.triangles[triangle1 as usize].update_neighbours(triangle2, triangle0, n1,
                                                             1, 0, idx_in_n1);
        self.triangles[triangle2 as usize].update_neighbours(triangle0, triangle1, n2,
                                                             1, 0, idx_in_n2);
        self.triangles[n0 as usize].update_neighbour(triangle0, 2, idx_in_n0);
        self.triangles[n1 as usize].update_neighbour(triangle1, 2, idx_in_n1);
        self.triangles[n2 as usize].update_neighbour(triangle2, 2, idx_in_n2);

        // Add new triangles to queue to check for delaunayness and check the criterion
        self.triangles_to_check.push_back(triangle0);
        self.triangles_to_check.push_back(triangle1);
        self.triangles_to_check.push_back(triangle2);

        self.fix_delaunayness();

        self.consistency_check();
    }

    pub fn to_str(&self) -> String {
        let mut result = String::from("# Vertices #\n");
        for (i, v) in self.vertices.iter().enumerate() {
            result += &format!("{}\t({}, {})\n", i, v.x, v.y);
        }

        result += "\n# Triangles #\n";
        for (i, triangle) in self.triangles[3..].iter().enumerate() {
            result += &format!("{}\t({}, {}, {})\n", i, triangle.vertices[0], triangle.vertices[1], triangle.vertices[2]);
        }
        result
    }

    pub fn to_file(&self, filename: &str) {
        fs::write(filename, self.to_str()).expect("Unable to write to file!");
    }

    fn new_vertex(&mut self, x: f64, y: f64) -> i32 {
        // TODO possibly manage the size of self.vertices more intelligently.
        self.current_vertex_idx = self.vertices.len() as i32;
        self.vertices.push(DelaunayVertex2D{x, y, ..DelaunayVertex2D::default()});

        self.current_vertex_idx
    }

    fn new_triangle(&mut self, v0: i32, v1: i32, v2: i32) -> i32 {
        self.new_triangle_at(v0, v1, v2, -1)
    }

    fn new_triangle_at(&mut self, v0: i32, v1: i32, v2: i32, mut at: i32) -> i32 {
        // TODO possibly manage the size of self.triangles more intelligently.
        if at < 0 {
            at = self.triangles.len() as i32;
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

    fn find_triangle_containing_current_vertex(&self) -> i32 {
        let vertex = &self.vertices[self.current_vertex_idx as usize];
        let mut v0: &DelaunayVertex2D;
        let mut v1: &DelaunayVertex2D;
        let mut v2: &DelaunayVertex2D;
        let mut current_triangle: &DelaunayTriangle2D;
        let mut current_triangle_idx = self.current_triangle_idx;
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
                panic!("Error! This scenario should not be possible?")
            }

            if current_triangle_idx < 3 {
                panic!("Ended up with dummy triangle!");
            }
        }
        current_triangle_idx
    }

    fn fix_delaunayness(&mut self) {
        while !self.triangles_to_check.is_empty() {
            let triangle_to_fix_idx = self.triangles_to_check.pop_front().unwrap();
            self.fix_triangle(triangle_to_fix_idx);
        }
    }

    fn fix_triangle(&mut self, triangle_idx: i32) {
        let triangle = &self.triangles[triangle_idx as usize];
        let neighbour_idx = triangle.neighbours[2];
        if neighbour_idx < 3 {
            return
        }

        let a = &self.vertices[triangle.vertices[0] as usize];
        let b = &self.vertices[triangle.vertices[1] as usize];
        let c = &self.vertices[triangle.vertices[2] as usize];

        let neighbour = &self.triangles[neighbour_idx as usize];
        let d = &self.vertices[neighbour.vertices[triangle.index_in_neighbours[2] as usize] as usize];

        let test = in_circle_2d(a.x, a.y, b.x, b.y, c.x, c.y, d.x, d.y);

        if test < 0. {
            self.flip_triangles(triangle_idx, neighbour_idx);
        }
    }

    fn flip_triangles(&mut self, triangle_idx: i32, neighbour_idx: i32) {
        let triangle = &self.triangles[triangle_idx as usize];
        let neighbour = &self.triangles[neighbour_idx as usize];

        // read necessary info
        let (ai, bi, ci) = (triangle.vertices[0], triangle.vertices[1], triangle.vertices[2]);
        let (na, nb) = (triangle.neighbours[0], triangle.neighbours[1]);
        let idx_in_na = triangle.index_in_neighbours[0];
        let idx_in_nb = triangle.index_in_neighbours[1];

        let idx_in_neighbour = triangle.index_in_neighbours[2] as usize;
        let fi = neighbour.vertices[idx_in_neighbour];
        let nd = neighbour.neighbours[(idx_in_neighbour + 1) % 3];
        let ne = neighbour.neighbours[(idx_in_neighbour + 2) % 3];
        let idx_in_nd = neighbour.index_in_neighbours[(idx_in_neighbour + 1) % 3];
        let idx_in_ne = neighbour.index_in_neighbours[(idx_in_neighbour + 2) % 3];

        // create new triangles
        let t0 = self.new_triangle_at(ai, fi, ci, triangle_idx);
        let t1 = self.new_triangle_at(fi, bi, ci, neighbour_idx);

        // fix neighbours
        self.triangles[t0 as usize].update_neighbours(t1, nb, nd, 1, idx_in_nb, idx_in_nd);
        self.triangles[t1 as usize].update_neighbours(na, t0, ne, idx_in_na, 0, idx_in_ne);
        self.triangles[na as usize].update_neighbour(t1, 0, idx_in_na);
        self.triangles[nd as usize].update_neighbour(t0, 2, idx_in_nd);
        // here the neighbours do not change, but idx_in_neighbour might change!
        self.triangles[nb as usize].update_neighbour(t0, 1, idx_in_nb);
        self.triangles[ne as usize].update_neighbour(t1, 2, idx_in_ne);

        self.triangles_to_check.push_back(t0);
        self.triangles_to_check.push_back(t1);

        // update current_triangle
        self.current_triangle_idx = t1;
    }

    pub(super) fn triangle_circumcenter(&self, triangle: &DelaunayTriangle2D) -> Vertex2D{
        circumcenter_2d(
            self.vertices[triangle.vertices[0] as usize].x,
            self.vertices[triangle.vertices[0] as usize].y,
            self.vertices[triangle.vertices[1] as usize].x,
            self.vertices[triangle.vertices[1] as usize].y,
            self.vertices[triangle.vertices[2] as usize].x,
            self.vertices[triangle.vertices[2] as usize].y
        )
    }

    fn consistency_check(&self) {
        if ! cfg!(debug_assertions) {
            return
        }
        // for each triangle check neighbour symmetry and check delaunay criterion
        // for each vertex check triangle information
        for (i, triangle) in self.triangles[3..].iter().enumerate() {
            let triangle_idx = (i + 3) as i32;
            for (j, &ngbr) in triangle.neighbours.iter().enumerate() {
                // check neighbouring relation symmetry
                let idx_in_ngbr = triangle.index_in_neighbours[j] as usize;
                assert_eq!(triangle_idx,
                           self.triangles[ngbr as usize].neighbours[idx_in_ngbr],
                           "Testing neighbour symmetry");

                // check overlapping vertices
                let neighbour = &self.triangles[ngbr as usize];
                let mut n_overlap = 0;
                for i in triangle.vertices.iter() {
                    if neighbour.vertices.contains(i) {
                        n_overlap += 1;
                    }
                }
                assert_eq!(n_overlap, 2, "Neighbours should have exactly 2 vertices in common!");

                // check Delaunay criterion
                // skip dummy triangles
                if ngbr < 3 {
                    continue;
                }
                let (a, b, c) = (&self.vertices[triangle.vertices[0] as usize],
                                 &self.vertices[triangle.vertices[1] as usize],
                                 &self.vertices[triangle.vertices[2] as usize]);
                let d = &self.vertices[neighbour.vertices[idx_in_ngbr] as usize];
                let in_circle = in_circle_2d(a.x, a.y, b.x, b.y, c.x, c.y, d.x, d.y);
                assert!(in_circle >= 0., "in_circle test gave {:?}", in_circle)
            }
        }
        for (i, vertex) in self.vertices.iter().enumerate() {
            assert_eq!(i as i32,
                       self.triangles[vertex.triangle as usize].vertices[vertex.index_in_triangle as usize],
                       "Testing vertex-triangle correspondence");
        }
        // println!("Consistency checks passed!");
    }
}