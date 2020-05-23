import math

N = int(input())

if N % 2 == 0:
    ans = 0
    for i in range(0, 26):
        ans += N // (5 ** i * 10)

    print(ans)
else:
    print(0)
