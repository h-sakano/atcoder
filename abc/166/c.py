N, M = map(int, input().split())
H = list(map(int, input().split()))
edges = {i + 1: [] for i in range(N)}
for i in range(M):
    A, B = map(int, input().split())
    edges[A].append(H[B - 1])
    edges[B].append(H[A - 1])


ans = 0
for i in range(N):
    if len(edges[i+1]) <= 0 or H[i] > max(edges[i+1]):
        ans += 1

print(ans)
