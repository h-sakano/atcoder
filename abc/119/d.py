import bisect
A, B, Q = map(int, input().split())

s = [-float('inf')] + [int(input()) for i in range(A)] + [float('inf')]
t = [-float('inf')] + [int(input()) for i in range(B)] + [float('inf')]

res = []
for i in range(Q):
    x = int(input())
    b = bisect.bisect_right(s, x)
    a = b - 1
    d = bisect.bisect_right(t, x)
    c = d - 1
    ans = float('inf')
    for S in [s[a], s[b]]:
        for T in [t[c], t[d]]:
            ans = min(ans, abs(S - x) + abs(T - S), abs(T - x) + abs(S - T))
    res.append(ans)

print('\n'.join(map(str, res)))
