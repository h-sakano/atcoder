import heapq

N = int(input())
A = []
for i in range(N):
    A.append(int(input()))

ans = []
heapq.heappush(ans, A[0])
for i in range(1, N):
    if A[i] > ans[0]:
        ans[0] = A[i]
    else:
        heapq.heappush(ans, A[i])

print(len(ans))
