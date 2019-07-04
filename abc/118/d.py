N, M = map(int, input().split())
A = list(map(int, input().split()))
A.sort(reverse=True)

NUM = [2, 5, 5, 4, 5, 6, 3, 7, 6]
dp = [-float('inf') for _ in range(N + 1)]
dp[0] = 0

for i in range(1, N + 1):
    for a in A:
        if i - NUM[a - 1] >= 0:
            dp[i] = max(dp[i], dp[i - NUM[a - 1]] + 1)

dights = dp[N]
ans = ""
matchsticks = N
while(dights > 0):
    for a in A:
        if matchsticks - NUM[a - 1] >= 0 and dp[matchsticks - NUM[a - 1]] == dights - 1:
            ans += str(a)
            dights -= 1
            matchsticks -= NUM[a - 1]
            break

print(ans)
