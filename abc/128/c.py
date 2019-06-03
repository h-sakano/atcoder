# -*- coding: utf-8 -*-
n, m = map(int, input().split())
s = []

for i in range(m):
    s.append(list(map(int, input().split())))

p = list(map(int, input().split()))

ans = 0
for value in range(1 << n):
    all = True
    for i in range(m):
        count = 0
        for j in range(1, s[i][0] + 1):
            count += ((1 << (s[i][j] - 1)) & value) != 0
        if count % 2 != p[i]:
            all = False
            break
    if all:
        ans += 1

print(ans)
