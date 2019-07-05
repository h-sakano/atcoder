N, M = map(int, input().split())

if N >= M:
    print(0)
else:
    X = list(map(int, input().split()))
    X.sort()

    diff = []

    for i in range(1, M):
        diff.append(X[i] - X[i - 1])
    diff.sort(reverse=True)

    print(sum(diff[N-1:]))
