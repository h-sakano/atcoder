# -*- coding: utf-8 -*-
import math

n = int(input())
trans = []
for i in range(5):
    trans.append(int(input()))

ans = 4 + math.ceil(n / min(trans))

print(ans)
