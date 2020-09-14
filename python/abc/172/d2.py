N = int(input())

ans = 0
for i in range(1, N + 1):
    start = i
    n = N // i
    end = i * n
    ans += n * (start + end) // 2


print(ans)
