N, M, K = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

# 全部Aから選んでみる
minutes_a = 0
count_a = 0
for i in range(N):
    if minutes_a + A[i] > K:
        break
    minutes_a += A[i]
    count_a += 1

ans = count_a
minutes_b = 0
count_b = 0

for i in range(M):
    if minutes_a + minutes_b + B[i] > K:
        found = False
        while count_a > 0:
            minutes_a -= A[count_a - 1]
            count_a -= 1
            if minutes_a + minutes_b + B[i] <= K:
                minutes_b += B[i]
                count_b += 1
                found = True
                break
        if not found:
            break
    else:
        minutes_b += B[i]
        count_b += 1
    ans = max(ans, count_a + count_b)

print(ans)
