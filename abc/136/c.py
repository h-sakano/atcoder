N = int(input())
H = list(map(int, input().split()))

ans = True
for i in range(N - 1, 0, -1):
    if H[i] - H[i - 1] == -1:
        H[i - 1] -= 1
    elif H[i] - H[i - 1] < -1:
        ans = False
        break

if ans:
    print('Yes')
else:
    print('No')
