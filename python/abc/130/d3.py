n, k = map(int, input().split())
a = list(map(int, input().split()))

ans = 0
total = 0
right = 0
for left in range(n):
    while total < k:
        if right >= n:
            break
        total += a[right]
        right += 1
    if total < k:
        break
    ans += n - right + 1
    total -= a[left]

print(ans)
