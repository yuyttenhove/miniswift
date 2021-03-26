from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.collections import PatchCollection
import seaborn as sns
from parse import parse
import pandas as pd
import numpy as np


def plot_tesselation(vertices, cells):
    ax = sns.scatterplot(data=vertices, x="x", y="y", s=8, color="blue")
    cells_xy_list = [vertices.values[cell_idx, :] for cell_idx in cells]
    patches = [plt.Polygon(xy, closed=False, edgecolor="blue", facecolor="none", linewidth=.5) for xy in cells_xy_list]
    ax.add_collection(PatchCollection(patches, match_original=True))
    ax.set_xlim([0, 1])
    ax.set_ylim([0, 1])
    ax.set_aspect("equal")
    plt.tight_layout()
    plt.savefig("voronoi.pdf")


def read_file(fname: Path):
    with open(fname, "r") as file:
        lines = file.read()
    lines = lines.split("# Vertices #")[-1]
    vertex_lines, cell_lines = lines.split("# Cells #")
    vertex_lines = [l for l in vertex_lines.split("\n") if len(l) > 0]
    cell_lines = [l for l in cell_lines.split("\n") if len(l) > 0]

    vertex_df = pd.DataFrame(index=range(len(vertex_lines)), columns=["x", "y"], dtype=float)
    for vertex_line in vertex_lines:
        res = parse("{}\t({}, {})", vertex_line)
        i = int(res[0])
        vertex_df.loc[i, "x"] = float(res[1])
        vertex_df.loc[i, "y"] = float(res[2])

    cell_list = []
    for triangle_line in cell_lines:
        res = parse("{}\t({})", triangle_line)
        vertices = np.array([int(i) for i in res[1].split(", ")])
        cell_list.append(vertices)

    return vertex_df, cell_list


def main():
    base_path = Path(__file__).parent
    fname = base_path / "voronoi.txt"
    vertices, cells = read_file(fname)

    plot_tesselation(vertices, cells)



if __name__ == "__main__":
    main()