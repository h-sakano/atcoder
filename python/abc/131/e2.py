N, K = map(int, input().split())

max_num = (N - 1) * (N - 2) // 2

if K > max_num:
    print(-1)
else:
    edges = []
    for i in range(2, N + 1):
        edges.append((1, i,))

    add = max_num - K

    candidates = []
    for i in range(2, N):
        for j in range(i + 1, N + 1):
            candidates.append((i, j,))

    for i in range(add):
        edges.append(candidates[i])

    print(len(edges))
    for edge in edges:
        print(' '.join(map(str, edge)))
