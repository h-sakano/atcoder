import bisect

N = int(input())
A = []
for i in range(N):
    A.append(int(input()))

ans = [A[0]]
for i in range(1, N):
    index = bisect.bisect_left(ans, A[i])
    if index > 0:
        ans[index - 1] = A[i]
    else:
        ans = [A[i]] + ans

print(len(ans))
