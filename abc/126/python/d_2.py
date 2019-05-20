# -*- coding: utf-8 -*-
n = int(input())
g = [[] for i in range(n)]
c = [-1 for i in range(n)]

for i in range(n - 1):
    u, v, w = map(int, input().split())
    u -= 1
    v -= 1
    g[u].append([v, w])
    g[v].append([u, w])


q = [0]
while (len(q) > 0):
    v = q.pop()
    for u, w in g[v]:
        if c[u] != -1:
            continue
        q.append(u)
        c[u] = (c[v] + w) % 2

for v in c:
    print(v)
