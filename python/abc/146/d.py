N = int(input())
colors = [set() for i in range(N)]
ans = [-1] * N

max_color = 1
color = 1
for i in range(N - 1):
    a, b = map(int, input().split())
    for c in range(1, max_color + 2):
        if c in colors[a - 1] or c in colors[b - 1]:
            continue
        color = c
        break
    max_color = max(max_color, color)
    colors[a - 1].add(color)
    colors[b - 1].add(color)
    ans[i] = color

print(max_color)
for i in range(N - 1):
    print(ans[i])
