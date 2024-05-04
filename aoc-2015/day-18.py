import numpy as np  # type: ignore
from itertools import product

with open(0) as f:
    lines = f.read().splitlines()

grid = np.full((len(lines) + 2, len(lines[0]) + 2), False)
for row in range(len(lines)):
    for col in range(len(lines[row])):
        if lines[row][col] == "#":
            grid[row + 1][col + 1] = True


def next_on(cell: bool, nbhs: int) -> bool:
    return (cell and nbhs in [3, 4]) or (not cell and nbhs == 3)


def next(grid: np.ndarray, corners_always_on=False) -> np.ndarray:
    next_grid = np.full_like(grid, False)

    for row, col in product(range(1, grid.shape[0] - 1), range(1, grid.shape[1] - 1)):
        nbhs = np.sum(grid[row - 1 : row + 2, col - 1 : col + 2])
        next_grid[row, col] = next_on(grid[row, col], nbhs)

    if corners_always_on:
        next_grid[1, 1] = True
        next_grid[1, -2] = True
        next_grid[-2, 1] = True
        next_grid[-2, -2] = True

    return next_grid


# For debugging only
def print_grid(grid: np.ndarray):
    grid = grid[1:-1, 1:-1]
    for row in range(grid.shape[0]):
        s = "".join(["#" if x else "." for x in grid[row]])
        print(s)


# -- Part One -- #

initial_grid = grid.copy()

for _ in range(100):
    grid = next(grid)

print(np.sum(grid))


# -- Part Two -- #

grid = initial_grid

for _ in range(100):
    grid = next(grid, corners_always_on=True)

print(np.sum(grid))
