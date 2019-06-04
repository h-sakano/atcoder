n, k = map(int, input().split())
v = list(map(int, input().split()))

ans = 0
for a in range(min(n, k) + 1):
    for b in range(min(n, k) - a + 1):
        li = v[:a]
        if b > 0:
            li += v[-b:]
        li.sort()
        count = 0
        while (a + b + count < k):
            if len(li) > 0 and li[0] < 0:
                count += 1
                li = li[1:]
            else:
                break
        ans = max(ans, sum(li))

print(ans)
