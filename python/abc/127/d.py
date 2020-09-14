# -*- coding: utf-8 -*-
n, m = map(int, input().split())
a = list(map(int, input().split()))
b = [-1] * m
c = [-1] * m
a.sort()
for j in range(m):
    b[j], c[j] = map(int, input().split())
    index = n
    for i in range(n):
        if c[j] < a[i]:
            index = i
            break
    for i in range(b[j]):
        a.insert(index, c[j])
    for i in range(b[j]):
        del a[0]

print(sum(a))