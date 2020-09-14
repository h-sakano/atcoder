# -*- coding: utf-8 -*-
n, m = map(int, input().split())

max_l = 0
min_r = n
for i in range(m):
    l, r = map(int, input().split())
    max_l = max(max_l, l)
    min_r = min(min_r, r)

res = min_r - max_l + 1

if res > 0:
    print(min_r - max_l + 1)
else:
    print(0)