from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.collections import PatchCollection
import seaborn as sns
from phd_python_scripts.utils.plotting_utils import subplots

from plot_delaunay import read_file as read_delaunay_file
from plot_voronoi import read_file as read_voronoi_file


def plot_tesselation(delaunay_vertices, delaunay_triangles, voronoi_vertices, voronoi_cells, voronoi_centroids):
    _, ax = subplots(figsize=(10, 10))
    ax = sns.scatterplot(data=delaunay_vertices, x="x", y="y", s=16, color="black", alpha=1, zorder=2, ax=ax)
    ax.triplot(delaunay_vertices["x"], delaunay_vertices["y"], triangles=delaunay_triangles, lw=1, color="darkgrey", alpha=.6, linestyle="--", zorder=-1)

    # ax = sns.scatterplot(data=voronoi_vertices, x="x", y="y", s=8, color="blue", ax=ax)
    # ax = sns.scatterplot(data=voronoi_centroids, x="x", y="y", s=16, color="red", ax=ax, marker="P")
    cells_xy_list = [voronoi_vertices.values[cell_idx, :] for cell_idx in voronoi_cells]
    patches = [plt.Polygon(xy, closed=True, edgecolor="blue", facecolor="none", linewidth=1) for xy in cells_xy_list]
    ax.add_collection(PatchCollection(patches, match_original=True))

    ax.set_xlim([.1, .9])
    ax.set_ylim([.1, .9])
    ax.get_xaxis().set_visible(False)
    ax.get_yaxis().set_visible(False)
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
