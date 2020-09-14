N = int(input())
A = list(map(int, input().split()))

B = [0] * N
ans = []
for i in range(N - 1, -1, -1):
    num = 0
    for j in range(2 * (i + 1), N + 1, i + 1):
        num += B[j - 1]
    if (num & 1) ^ A[i]:
        B[i] = 1
        ans.append(i + 1)

count = len(ans)
print(count)
if count > 0:
    print(' '.join(map(str, ans)))
