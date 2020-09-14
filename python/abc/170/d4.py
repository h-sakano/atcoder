# エラトステネスの篩

N = int(input())
A = sorted(map(int, input().split()))
maxA = A[N - 1]
dp = [0] * (maxA + 1)

for i in range(len(A)):
    cursor = A[i]
    while cursor <= maxA:
        dp[cursor] += 1
        cursor += A[i]

ans = 0
for a in A:
    if dp[a] == 1:
        ans += 1

print(ans)
