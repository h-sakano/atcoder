import numpy as np

h, w = map(int, input().split())
grid = [[False for j in range(w)] for i in range(h)]

for i in range(h):
    s = input()
    for j in range(w):
        if s[j] == '.':
            grid[i][j] = 1
        else:
            grid[i][j] = 0

grid = np.array(grid)

L = np.zeros((h, w), dtype=int)
R = np.zeros((h, w), dtype=int)
U = np.zeros((h, w), dtype=int)
D = np.zeros((h, w), dtype=int)

for j in range(w):
    if j == 0:
        L[:, j] = grid[:, j]
    else:
        L[:, j] = (L[:, j - 1] + 1) * grid[:, j]


for j in range(w - 1, -1, -1):
    if j >= w - 1:
        R[:, j] = grid[:, j]
    else:
        R[:, j] = (R[:, j + 1] + 1) * grid[:, j]

for i in range(h):
    if i <= 0:
        U[i, :] = grid[i, :]
    else:
        U[i, :] = (U[i - 1, :] + 1) * grid[i, :]


for i in range(h - 1, -1, -1):
    if i >= h - 1:
        D[i, :] = grid[i, :]
    else:
        D[i, :] = (D[i + 1, :] + 1) * grid[i, :]

print(np.max(L + R + U + D - 3))
