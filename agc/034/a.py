# -*- coding: utf-8 -*-
n, a, b, c, d = map(int, input().split())
s = input()
grid = []
for rock in s:
    grid.append(rock == '#')

if grid[c-1] or grid[d-1]:
    print('No')

max_rock_count = 0
rock_count = 0
if c < d:
    for i in range(a-1, d):
        if grid[i]:
            rock_count += 1
        else:
            rock_count = 0

        max_rock_count = max(max_rock_count, rock_count)

    if max_rock_count >= 2:
        print('No')
    else:
        print('Yes')
elif c > d:
    max_rock_count = 0
    rock_count = 0
    canSkip = False
    for i in range(a-1, c):
        if grid[i]:
            rock_count += 1
        else:
            if b - 1 <= i and i <= c - 2 and not grid[i-1] and not grid[i] and not grid[i+1]:
                if i-1 != d - 1:
                    canSkip = True
            rock_count = 0

        max_rock_count = max(max_rock_count, rock_count)

    if max_rock_count <= 1 and canSkip:
        print('Yes')
    else:
        print('No')
