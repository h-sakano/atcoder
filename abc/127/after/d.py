# -*- coding: utf-8 -*-
n, m = map(int, input().split())
a = list(map(int, input().split()))
a.sort()
bc = [tuple(map(int, input().split())) for _ in range(m)]
bc.sort(key=lambda t: t[1], reverse=True)

index = 0
count = [0] * m
for i in range(n):
    if index >= m or a[i] >= bc[index][1]:
        break
    a[i] = bc[index][1]
    count[index] += 1
    if count[index] >= bc[index][0]:
        index += 1

print(sum(a))
