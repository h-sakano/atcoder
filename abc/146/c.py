A, B, X = map(int, input().split())
N = X


def dfs(start, end):
    if start >= end:
        return start
    x = start + (end - start) // 2 + 1
    if A * x + B * len(str(x)) <= X:
        return dfs(x, end)
    else:
        return dfs(start, x - 1)


print(min(10 ** 9, dfs(0, N)))
