# -*- coding: utf-8 -*-
A, B = map(int, input().split())

res = 0
if A <= 5:
    res = 0
elif 6 <= A and A <= 12:
    res = B / 2
else:
    res = B

print(int(res))