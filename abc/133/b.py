import math
N, D = map(int, input().split())

X = [[] for _ in range(N)]
for i in range(N):
    X[i] = list(map(int, input().split()))

ans = 0
for i in range(N - 1):
    for j in range(i + 1, N):
        dist = 0
        for d in range(D):
            dist = dist + (X[i][d] - X[j][d]) ** 2
        dist = math.sqrt(dist)
        if dist.is_integer():
            ans += 1

print(ans)
