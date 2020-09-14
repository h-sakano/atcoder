N = int(input())
A = []
for i in range(N):
    A.append(int(input()))

ans = [A[0]]
for i in range(1, N):
    tmp = (-1, -1)
    for j in range(len(ans)):
        if ans[j] < A[i] and ans[j] > tmp[1]:
            tmp = (j, ans[j])

    if tmp[0] >= 0:
        ans[tmp[0]] = A[i]
    else:
        ans.append(A[i])

print(len(ans))
