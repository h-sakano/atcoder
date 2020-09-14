# コンテスト終了後の復習

N = int(input())
edges = [[] for i in range(N)]
ans = [-1] * (N - 1)
lines = []

for i in range(N - 1):
    a, b = map(int, input().split())
    lines.append((a, b))
    edges[a - 1].append((b - 1, i,))

q = [(0, 0,)]
max_color = 0
while q:
    v, c = q.pop()
    color = 1
    if c == color:
        color += 1
    for nv, i in edges[v]:
        ans[i] = color
        q.append((nv, color,))
        color += 1
        if c == color:
            color += 1

print(max(ans))
for i in range(N - 1):
    a, b = lines[i]
    print(ans[i])
