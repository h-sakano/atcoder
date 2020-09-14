# -*- coding: utf-8 -*-
n = int(input())
restaurants = []
for i in range(n):
    s, p = input().split()
    restaurants.append((s, -int(p), i))
restaurants = sorted(restaurants)
for r in restaurants:
    print(r[2] + 1)
