# -*- coding: utf-8 -*-
time = []
for i in range(5):
    time.append(int(input()))


def ten_time(x):
    return (x + 9) // 10 * 10


def rem(x):
    return ten_time(x) - x


ans = 0
for i in range(len(time)):
    ans += ten_time(time[i])

ans -= rem(max(time, key=rem))

print(ans)
