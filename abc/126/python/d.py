# -*- coding: utf-8 -*-
import sys
sys.setrecursionlimit(20000000)

n = int(input())
g = [[] for i in range(n)]
c = [-1 for i in range(n)]

for i in range(n - 1):
    u, v, w = map(int, input().split())
    u -= 1
    v -= 1
    w = w % 2
    g[u].append([v, w])
    g[v].append([u, w])


def dfs(p, d):
    if c[p] == -1:
        c[p] = d
        for x, w in g[p]:
            dfs(x, d ^ w)


dfs(0, 0)

for i in range(len(c)):
    print(c[i])
