from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.collections import PatchCollection
import seaborn as sns
from parse import parse
import pandas as pd
import numpy as np
from phd_python_scripts.utils.plotting_utils import subplots


def clean_lines(lines):
    return [l for l in lines.split("\n") if len(l) > 0]


def read_points_from_lines(lines):
    df = pd.DataFrame(index=range(len(lines)), columns=["x", "y"], dtype=float)
    for line in lines:
        res = parse("{}\t({}, {})", line)
        i = int(res[0])
        df.loc[i, "x"] = float(res[1])
        df.loc[i, "y"] = float(res[2])
    return df


def plot_tesselation(vertices, cells, centroids, fname):
    _, ax = subplots(figsize=(10, 10))
    # ax = sns.scatterplot(data=centroids, x="x", y="y", s=16, color="red", ax=ax, marker="P")
    # ax = sns.scatterplot(data=vertices, x="x", y="y", s=8, color="blue", ax=ax)
    cells_xy_list = [vertices.values[cell_idx, :] for cell_idx in cells]
    patches = [plt.Polygon(xy, closed=True, edgecolor="blue", facecolor="none", linewidth=1) for xy in cells_xy_list]

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
    patches.extend([plt.Polygon(g, closed=True, edgecolor="red", facecolor="none", linewidth=1, ls="--") for g in grid])
    ax.add_collection(PatchCollection(patches, match_original=True))

    ax.set_xlim([-.5, 2.5])
    ax.set_ylim([-.5, 2.5])
    ax.get_xaxis().set_visible(False)
    ax.get_yaxis().set_visible(False)
    ax.set_aspect("equal")
    plt.tight_layout()
    plt.savefig(fname)


def read_file(fname: Path):
    with open(fname, "r") as file:
        lines = file.read()
    lines = lines.split("# Vertices #")[-1]
    vertex_lines, cell_lines = lines.split("# Cells #")
    cell_lines, centroid_lines = cell_lines.split("# Centroids #")
    vertex_lines = clean_lines(vertex_lines)
    cell_lines = clean_lines(cell_lines)
    centroid_lines = clean_lines(centroid_lines)

    vertex_df = read_points_from_lines(vertex_lines)
    centroid_df = read_points_from_lines(centroid_lines)

    cell_list = []
    for triangle_line in cell_lines:
        res = parse("{}\t({})", triangle_line)
        vertices = np.array([int(i) for i in res[1].split(", ")])
        cell_list.append(vertices)

    return vertex_df, cell_list, centroid_df


def main():
    base_path = Path(__file__).parent / "output"
    for suffix in ["_i3", "_j", "_k", "_l3"]:
        basename = f"vor{suffix}"
        fname = base_path / f"{basename}.txt"
        vertices, cells, centroid_df = read_file(fname)
        plot_tesselation(vertices, cells, centroid_df, base_path / f"{basename}.pdf")


if __name__ == "__main__":
    main()
