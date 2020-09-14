import heapq

N = int(input())

works = []
for i in range(N):
    a, b = map(int, input().split())
    heapq.heappush(works, (b, a,))

time = 0
can = True
for i in range(N):
    work = heapq.heappop(works)
    time += work[1]
    if time > work[0]:
        can = False
        break

if can:
    print('Yes')
else:
    print('No')
