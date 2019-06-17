n, k = map(int, input().split())
a = list(map(int, input().split()))

summary = [0]
for i in range(n):
    summary.append(summary[i] + a[i])

cnt = 0
for right in range(n, 0, - 1):
    if summary[right] <= k:
        break
    for left in range(0, right):
        if summary[right] - summary[left + 1] >= k:
            cnt += 1
        else:
            break

print(cnt)
