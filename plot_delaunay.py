from pathlib import Path
import matplotlib.pyplot as plt
import seaborn as sns
from parse import parse
import pandas as pd
import numpy as np


def plot_tesselation(vertices, triangles):
    ax = sns.scatterplot(data=vertices, x="x", y="y", s=8, color="black")
    ax.triplot(vertices["x"], vertices["y"], triangles=triangles, lw=.5, color="black")
    ax.set_xlim([0, 1])
    ax.set_ylim([0, 1])
    ax.set_aspect("equal")
    plt.tight_layout()
    plt.savefig("test.pdf")


def read_file(fname: Path):
    with open(fname, "r") as file:
        lines = file.read()
    lines = lines.split("# Vertices #")[-1]
    vertex_lines, triangle_lines = lines.split("# Triangles #")
    vertex_lines = [l for l in vertex_lines.split("\n") if len(l) > 0]
    triangle_lines = [l for l in triangle_lines.split("\n") if len(l) > 0]

    vertex_df = pd.DataFrame(index=range(len(vertex_lines)), columns=["x", "y"], dtype=float)
    for vertex_line in vertex_lines:
        res = parse("{}\t({}, {})", vertex_line)
        i = int(res[0])
        vertex_df.loc[i, "x"] = float(res[1])
        vertex_df.loc[i, "y"] = float(res[2])

    triangle_array = np.zeros((len(triangle_lines), 3), dtype=int)
    for triangle_line in triangle_lines:
        res = parse("{}\t({}, {}, {})", triangle_line)
        i = int(res[0])
        triangle_array[i, 0] = int(res[1])
        triangle_array[i, 1] = int(res[2])
        triangle_array[i, 2] = int(res[3])

    return vertex_df, triangle_array


def main():
    base_path = Path(__file__).parent
    fname = base_path / "test.txt"
    vertices, triangles = read_file(fname)

    plot_tesselation(vertices, triangles)



if __name__ == "__main__":
    main()