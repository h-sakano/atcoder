# -*- coding: utf-8 -*-
h, w = map(int, input().split())
a = []
a = [[True] * (w + 2)]
for i in range(h):
    a.append([True] + [x == '#' for x in input()] + [True])
a.append([True] * (w + 2))
q = []

for i in range(1, h + 1):
    for j in range(1, w + 1):
        if a[i][j]:
            q.append((i, j))

count = 0
while True:
    qq = []
    for i, j in q:
        if not a[i-1][j]:
            a[i-1][j] = True
            qq.append((i - 1, j))
        if not a[i+1][j]:
            a[i+1][j] = True
            qq.append((i + 1, j))
        if not a[i][j-1]:
            a[i][j-1] = True
            qq.append((i, j - 1))
        if not a[i][j+1]:
            a[i][j+1] = True
            qq.append((i, j + 1))
    q = qq
    if len(q) <= 0:
        break
    count += 1

print(count)
