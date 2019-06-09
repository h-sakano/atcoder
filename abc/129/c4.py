n, m = map(int, input().split())
mod = 10 ** 9 + 7
pat = [0] * (n + 2)
pat[0] = 1
pat[1] = 1

for i in range(m):
    a = int(input())
    pat[a] = -1

for i in range(2, n + 1):
    if pat[i] < 0:
        continue
    if pat[i - 1] > 0:
        pat[i] = (pat[i] + pat[i - 1]) % mod
    if pat[i - 2] > 0:
        pat[i] = (pat[i] + pat[i - 2]) % mod

print(pat[n])
