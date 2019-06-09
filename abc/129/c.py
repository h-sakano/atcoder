n, m = map(int, input().split())
mod = 10 ** 9 + 7
a = []
for i in range(m):
    a.append(int(input()))

ans = 1
for i in range(n - 1):
    if i in a:
        continue
    cnt = 0
    if i + 1 <= n and i + 1 not in a:
        cnt = 1
    if i + 2 <= n and i + 2 not in a:
        cnt += 1
    ans = ans * cnt % mod
    if ans == 0:
        break

print(ans)
