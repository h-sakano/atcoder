N = int(input())
a = list(map(int, input().split()))

ans = [0] * N
bit = 0
for i in range(1, N):
    ans[i] = a[i - 1] ^ a[i] ^ ans[i - 1]
    bit ^= ans[i]

if bit == a[0]:
    print(' '.join(map(str, ans)))
else:
    index = 0
    for i in range(1, 10 ** 9 + 1):
        bit = bit ^ (i - 1) ^ i
        if bit == a[0]:
            index = i
            break

    ans[0] = index
    for i in range(1, N):
        ans[i] = a[i - 1] ^ a[i] ^ ans[i - 1]
        bit ^= ans[i]

    print(' '.join(map(str, ans)))
