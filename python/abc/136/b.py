import math

N = int(input())
log = int(math.log(N, 10))

ans = 0
for i in range(log):
    if i % 2 == 0:
        ans += 10 ** (i + 1) - 10 ** i

if log % 2 == 0:
    ans += N + 1 - (10 ** log)

print(ans)
