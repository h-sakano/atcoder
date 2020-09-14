N = int(input())
h = list(map(int, input().split()))

cnt = 0
for i in range(100, 0, -1):
    prev = False
    for j in range(N):
        if h[j] == i:
            if not prev:
                cnt += 1
            h[j] -= 1
            prev = True
        else:
            prev = False

print(cnt)
