import heapq

N, M = map(int, input().split())
A = list(map(int, input().split()))
pA = []

for a in A:
    heapq.heappush(pA, -a)

for m in range(M):
    a = heapq.heappop(pA)
    heapq.heappush(pA, -(-a // 2))

print(-sum(pA))
