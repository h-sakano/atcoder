n, m = map(int, input().split())
mod = 10 ** 9 + 7
a = []
for i in range(m):
    a.append(int(input()))

pat = [1]
zero = False
for i in range(1, n + 1):
    if i - 1 in a and i - 2 in a:
        zero = True
        break
    if i in a:
        pat.append(0)
        continue
    cnt = 0
    if i - 1 not in a:
        cnt = (cnt + pat[i - 1]) % mod
    if i - 2 >= 0 and i - 2 not in a:
        cnt = (cnt + pat[i - 2]) % mod
    pat.append(cnt)

if zero:
    print(0)
else:
    print(pat[n])
