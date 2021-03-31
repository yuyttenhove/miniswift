# rust-voronoi
A library to compute delaunay triangulations and the dual Voronoi tesselation for a set of generators in 2D or 3D (TODO).
This code is largely based on Bert Vandenbroecke's implementation in cVoronoi.

## Assumptions:
These assumptions are enforced and used while building the Delaunay triangulation and constructing the dual Voronoi mesh
- The points in any triangle in a Delaunay-triangulation are always listed counterclockwise.
- The last point in any triangle in a Delaunay-triangulation is the point of that triangle that was last added to the triangulation. 
- The neighbourlist of any triangle in a Delaunay-triangulation is in the order of the opposing corners

## TODO
- [ ] Fix Lloyd's relaxation
- [ ] Use square starting triangulation?
- [ ] Use arbitrary precision in geometry calculations Delaunay triangulation
- [ ] Periodic boundary conditions
- [ ] Treat non-periodic case correctly
- [ ] 3D