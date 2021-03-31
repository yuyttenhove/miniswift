from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.collections import PatchCollection
import seaborn as sns

from plot_delaunay import read_file as read_delaunay_file
from plot_voronoi import read_file as read_voronoi_file


def plot_tesselation(delaunay_vertices, delaunay_triangles, voronoi_vertices, voronoi_cells, voronoi_centroids):
    ax = sns.scatterplot(data=delaunay_vertices, x="x", y="y", s=8, color="grey", alpha=.75)
    ax.triplot(delaunay_vertices["x"], delaunay_vertices["y"], triangles=delaunay_triangles, lw=.5, color="grey", alpha=.5, linestyle="--")

    ax = sns.scatterplot(data=voronoi_vertices, x="x", y="y", s=8, color="blue", ax=ax)
    ax = sns.scatterplot(data=voronoi_centroids, x="x", y="y", s=8, color="red", ax=ax, marker="P")
    cells_xy_list = [voronoi_vertices.values[cell_idx, :] for cell_idx in voronoi_cells]
    patches = [plt.Polygon(xy, closed=True, edgecolor="blue", facecolor="none", linewidth=.5) for xy in cells_xy_list]
    ax.add_collection(PatchCollection(patches, match_original=True))

    ax.set_xlim([0, 1])
    ax.set_ylim([0, 1])
    ax.set_aspect("equal")
    plt.tight_layout()
    plt.savefig("both.pdf")


def main():
    base_path = Path(__file__).parent
    delaunay_vertices, delaunay_triangles = read_delaunay_file(base_path / "delaunay.txt")
    voronoi_vertices, voronoi_cells, voronoi_centroids = read_voronoi_file(base_path / "voronoi.txt")

    plot_tesselation(delaunay_vertices, delaunay_triangles, voronoi_vertices, voronoi_cells, voronoi_centroids)


if __name__ == "__main__":
    main()
