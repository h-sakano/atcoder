N = int(input())
a = list(map(int, input().split()))
S = 0
for i in range(N):
    S ^= a[i]

ans = [0] * N

for i in range(N):
    ans[i] = S ^ a[i]

print(' '.join(map(str, ans)))
