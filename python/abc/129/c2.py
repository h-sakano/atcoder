n, m = map(int, input().split())
mod = 10 ** 9 + 7
a = []
for i in range(m):
    a.append(int(input()))

dp = [-1] * (n + 1)


def dfs(step):
    if dp[step] >= 0:
        return dp[step]
    if step - 1 in a and step - 2 in a:
        return 0
    cnt = 0
    if step <= 0:
        return 1
    if step - 1 >= 0 and step - 1 not in a:
        cnt = (cnt + dfs(step - 1)) % mod
    if step - 2 >= 0 and step - 2 not in a:
        cnt = (cnt + dfs(step - 2)) % mod
    dp[step] = cnt
    return dp[step]


print(dfs(n))
