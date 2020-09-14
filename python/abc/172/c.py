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

minutes_b = 0
count_b = 0

if minutes_a < K:
    for i in range(M):
        if minutes_a + minutes_b + B[i] > K:
            break
        minutes_b += B[i]
        count_b += 1


max_count = count_a + count_b

for i in range(count_b, M):
    if count_a > 0:
        minutes_a -= A[count_a - 1]
        count_a -= 1
    if minutes_a + minutes_b + B[i] > K:
        break
    minutes_b += B[i]
    count_b += 1

print(max(max_count, count_a + count_b))
