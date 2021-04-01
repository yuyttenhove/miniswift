# rust-voronoi
A library to compute delaunay triangulations and the dual Voronoi tesselation for a set of generators in 2D or 3D (TODO).
This code is largely based on Bert Vandenbroecke's implementation in cVoronoi (see: https://github.com/bwvdnbro/cVoronoi).

## Assumptions:
These assumptions are enforced and used while building the Delaunay triangulation and constructing the dual Voronoi mesh
- The points in any triangle in a Delaunay-triangulation are always listed counterclockwise.
- The last point in any triangle in a Delaunay-triangulation is the point of that triangle that was last added to the triangulation. 
- The neighbourlist of any triangle in a Delaunay-triangulation is in the order of the opposing corners
- The last created triangle is the first tested triangle when inserting a new point

## TODO
- [X] Fix Lloyd's relaxation
- [ ] Use square starting triangulation? -> Better precision?
- [ ] Use arbitrary precision in geometry calculations Delaunay triangulation
- [X] Periodic boundary conditions
- [ ] Make parameter of number of dummy triangles and vertices
- [ ] Treat non-periodic case correctly -> bound voronoi cells by SimulationDomain
- [ ] 3D
- [ ] embed Vertex2D in DelaunayVertex2D and Triangle2D in DelaunayTriangle2D? -> store plain triangles and vertices in 
  seperate lists in DelaunayTriangulation2D and refer to those using indices?
- [ ] Insert vertices in Hilbert order
