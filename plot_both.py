from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.collections import PatchCollection
import seaborn as sns
import numpy as np
from phd_python_scripts.utils.plotting_utils import subplots

from plot_delaunay import read_file as read_delaunay_file
from plot_voronoi import read_file as read_voronoi_file


def plot_tesselation(delaunay_vertices, delaunay_triangles, voronoi_vertices, voronoi_cells, voronoi_centroids, fname):
    _, ax = subplots(figsize=(10, 10))
    ax = sns.scatterplot(data=delaunay_vertices, x="x", y="y", s=4, color="black", alpha=1, zorder=2, ax=ax)
    ax.triplot(delaunay_vertices["x"], delaunay_vertices["y"], triangles=delaunay_triangles, lw=.5, color="darkgrey", alpha=.6, linestyle="--", zorder=-1)

    # ax = sns.scatterplot(data=voronoi_vertices, x="x", y="y", s=8, color="blue", ax=ax)
    # ax = sns.scatterplot(data=voronoi_centroids, x="x", y="y", s=16, color="red", ax=ax, marker="P")
    cells_xy_list = [voronoi_vertices.values[cell_idx, :] for cell_idx in voronoi_cells]
    patches = [plt.Polygon(xy, closed=True, edgecolor="blue", facecolor="none", linewidth=.5) for xy in cells_xy_list]
    grid = [
        np.array([[0, 0], [.5, 0], [.5, .5], [0, .5]]),  # ci
        np.array([[.5, 0], [1, 0], [1, .5], [.5, .5]]),  # ci
        np.array([[0, .5], [.5, .5], [.5, 1], [0, 1]]),  # ci
        np.array([[.5, .5], [1, .5], [1, 1], [.5, 1]]),  # ci
        np.array([[1, 0], [2, 0], [2, 1], [1, 1]]),  # cj
        np.array([[0, 1], [1, 1], [1, 2], [0, 2]]),  # ck
        np.array([[1, 1], [1.5, 1], [1.5, 1.5], [1, 1.5]]),  # cl
        np.array([[1.5, 1], [2, 1], [2, 1.5], [1.5, 1.5]]),  # cl
        np.array([[1, 1.5], [1.5, 1.5], [1.5, 2], [1, 2]]),  # cl
        np.array([[1.5, 1.5], [2, 1.5], [2, 2], [1.5, 2]]),  # cl
    ]
    patches.extend([plt.Polygon(g, closed=True, edgecolor="red", facecolor="none", linewidth=.5, ls="--") for g in grid])
    ax.add_collection(PatchCollection(patches, match_original=True))

    ax.set_xlim([-.5, 2.5])
    ax.set_ylim([-.5, 2.5])
    ax.get_xaxis().set_visible(False)
    ax.get_yaxis().set_visible(False)
    ax.set_aspect("equal")
    plt.tight_layout()
    plt.savefig(fname)


def main():
    base_path = Path(__file__).parent / "output"
    for suffix in ["_i0", "_i1", "_i2", "_i3", "_j", "_k", "_l0", "_l1", "_l2", "_l3"]:
        delaunay_vertices, delaunay_triangles = read_delaunay_file(base_path / f"del{suffix}.txt")
        voronoi_vertices, voronoi_cells, voronoi_centroids = read_voronoi_file(base_path / f"vor{suffix}.txt")

        plot_tesselation(delaunay_vertices, delaunay_triangles, voronoi_vertices, voronoi_cells, voronoi_centroids, base_path / f"both{suffix}.pdf")


if __name__ == "__main__":
    main()
