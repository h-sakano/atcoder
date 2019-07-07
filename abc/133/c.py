L, R = map(int, input().split())
mod = 2019
left = L // mod
right = R // mod
l_mod = L % mod
r_mod = R % mod

if left != right or l_mod == 0 or r_mod == 0:
    print(0)
else:
    ans = float('inf')
    for i in range(l_mod, r_mod):
        for j in range(i + 1, r_mod + 1):
            ans = min(ans, (i * j) % mod)
    print(ans)
