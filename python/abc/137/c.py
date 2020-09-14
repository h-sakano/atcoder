N = int(input())
S = [''.join(sorted(input())) for _ in range(N)]
cnt = {}
ans = 0

for i in range(N):
    num = cnt.get(S[i], 0)
    ans += num
    cnt[S[i]] = num + 1

print(ans)
