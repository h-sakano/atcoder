n, k = map(int, input().split())
a = list(map(int, input().split()))

summary = sum(a)

cnt = 0
for left in range(n):
    tmp = summary
    for right in range(n, left, - 1):
        if tmp >= k:
            cnt += 1
            if right - 1 >= 0:
                tmp -= a[right - 1]
        else:
            break
    if left <= n - 1:
        summary -= a[left]

print(cnt)
