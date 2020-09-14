N = int(input())
a = list(map(int, input().split()))

ans = [0] * N

for i in range(10 ** 9 + 1):
    ans[0] = i
    bit = 0
    for i in range(1, N):
        ans[i] = a[i - 1] ^ a[i] ^ ans[i - 1]
        bit ^= ans[i]
    if bit == a[0]:
        break

print(' '.join(map(str, ans)))
