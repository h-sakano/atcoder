import heapq

x, y, z, k = map(int, input().split())
a = list(map(int, input().split()))
b = list(map(int, input().split()))
c = list(map(int, input().split()))

sums = []
for ai in a:
    for bi in b:
        for ci in c:
            heapq.heappush(sums, -(ai + bi + ci))

cnt = 0
while (cnt < k):
    print(-heapq.heappop(sums))
    cnt += 1
