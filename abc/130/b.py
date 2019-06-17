n, x = map(int, input().split())
L = list(map(int, input().split()))

ans = 1
d = 0
while True:
    if ans >= n + 1:
        break
    d += L[ans - 1]
    if d > x:
        break

    ans += 1

print(ans)
