# -*- coding: utf-8 -*-
from heapq import heappush, heappop

qn = int(input())

low = []
high = []
ans = 0

for i in range(qn):
    q = [int(i) for i in input().split()]
    if q[0] == 1:
        ans += q[2]
        a = q[1]

        if len(low) >= 1:
            if a < -low[0]:
                ans += -low[0] - a
            elif a > high[0]:
                ans += a - high[0]

        # lowは降順に並べたいので-aを入れる
        heappush(low, -a)
        heappush(high, a)
        if -low[0] > high[0]:
            low0 = heappop(low)
            high0 = heappop(high)
            # lowとhighで正負が逆なので注意する
            heappush(low, -high0)
            heappush(high, -low0)

    elif q[0] == 2:
        print(-low[0], ans)
