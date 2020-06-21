from collections import Counter

N = int(input())
A = list(map(int, input().split()))
Q = int(input())
counter = Counter(A)
ans = sum(A)

for i in range(Q):
    B, C = map(int, input().split())
    ans += (C - B) * counter[B]
    counter[C] += counter[B]
    counter[B] = 0
    print(ans)
