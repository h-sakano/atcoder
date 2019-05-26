# -*- coding: utf-8 -*-
r, d, x2000 = map(int, input().split())
x = [x2000]
for i in range(1, 11):
    xi = r * x[i - 1] - d
    x.append(xi)
    print(xi)