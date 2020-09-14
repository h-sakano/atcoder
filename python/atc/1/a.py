import sys
sys.setrecursionlimit(10**7)

H, W = map(int, input().split())
streets = []
for i in range(H):
    street = list(input())
    streets.append(street)

history = [[False for w in range(W)] for h in range(H)]


def dfs(x, y):
    if x < 0 or y < 0 or x >= W or y >= H or streets[y][x] == '#' or history[y][x]:
        return False
    if streets[y][x] == 'g':
        return True
    history[y][x] = True
    ret1 = dfs(x + 1, y)
    ret2 = dfs(x - 1, y)
    ret3 = dfs(x, y + 1)
    ret4 = dfs(x, y - 1)
    return ret1 or ret2 or ret3 or ret4


ans = False
for y in range(H):
    for x in range(W):
        if streets[y][x] == 's':
            ans = dfs(x, y)
            break

if ans:
    print('Yes')
else:
    print('No')
