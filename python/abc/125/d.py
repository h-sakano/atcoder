# -*- coding: utf-8 -*-
n = int(input())
a = list(map(int, input().split()))

count = 0
for i in range(n):
    if a[i] < 0:
        a[i] = -a[i]
        count += 1

ans = sum(a)
if count % 2 == 0:
    print(ans)
else:
    print(ans - min(a) * 2)
