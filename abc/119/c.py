N, A, B, C = map(int, input().split())
L = [int(input()) for i in range(N)]


def dfs(cur, a, b, c):
    if cur >= N:
        return abs(A - a) + abs(B - b) + abs(C - c) - 30 if min(a, b, c) > 0 else float('inf')
    res0 = dfs(cur + 1, a, b, c)
    res1 = dfs(cur + 1, a + L[cur], b, c) + 10
    res2 = dfs(cur + 1, a, b + L[cur], c) + 10
    res3 = dfs(cur + 1, a, b, c + L[cur]) + 10
    return min(res0, res1, res2, res3)


print(dfs(0, 0, 0, 0))
