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

for i in range(count_b, M):
    if minutes_a + minutes_b + B[i] > K:
        if count_a > 0 and minutes_a - A[count_a - 1] + minutes_b + B[i] <= K:
            minutes_a -= A[count_a - 1]
            count_a -= 1
            minutes_b += B[i]
            count_b += 1
            continue
        else:
            break
    minutes_b += B[i]
    count_b += 1

a_max = count_a + count_b

# 全部Bから選んでみる
minutes_b = 0
count_b = 0
for i in range(M):
    if minutes_b + B[i] > K:
        break
    minutes_b += B[i]
    count_b += 1

minutes_a = 0
count_a = 0

if minutes_b < K:
    for i in range(N):
        if minutes_b + minutes_a + A[i] > K:
            break
        minutes_a += A[i]
        count_a += 1

for i in range(count_a, N):
    if minutes_b + minutes_a + A[i] > K:
        if count_b > 0 and minutes_b - B[count_b - 1] + minutes_a + A[i] <= K:
            minutes_b -= B[count_a - 1]
            count_b -= 1
            minutes_a += A[i]
            count_a += 1
            continue
        else:
            break
    minutes_a += A[i]
    count_a += 1

print(max(a_max, count_a + count_b))
