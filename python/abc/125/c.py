# -*- coding: utf-8 -*-
def gcd(a, b):
    if (b == 0):
        return a
    return gcd(b, a % b)


n = int(input())
a = list(map(int, input().split()))
li = [0] * (n + 1)
r = [0] * (n + 1)

for i in range(n):
    li[i + 1] = gcd(li[i], a[i])
    r[n - i - 1] = gcd(r[n - i], a[n - i - 1])

ans = 0
for i in range(n):
    ans = max(ans, gcd(li[i], r[i + 1]))

print(ans)
