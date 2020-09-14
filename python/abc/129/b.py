n = int(input())
w = list(map(int, input().split()))

ans = float('inf')
s1 = 0
s2 = sum(w)

for i in range(n):
    s1 += w[i]
    s2 -= w[i]
    ans = min(ans, abs(s2 - s1))

print(ans)
