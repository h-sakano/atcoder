N = int(input())

dp = [0] * (N + 1)
for i in range(1, N + 1):
    cursor = i
    while cursor <= N:
        dp[cursor] += cursor
        cursor += i

print(sum(dp))
