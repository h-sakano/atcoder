import heapq

n, m = map(int, input().split())

shops = []
for i in range(n):
    a, b = map(int, input().split())
    heapq.heappush(shops, (a, b))

cnt = 0
ans = 0
while True:
    a, b = heapq.heappop(shops)
    if cnt + b >= m:
        ans += a * (m - cnt)
        cnt += m - cnt
        break
    else:
        ans += a * b
        cnt += b

print(ans)
